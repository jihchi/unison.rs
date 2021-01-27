use super::env;
use crate::env::Result;
use shared::types::*;
use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};

// fn filter_free_vbls(
//     free: &Vec<(Symbol, usize, usize, bool)>,
//     names: &Vec<(Symbol, usize)>,
// ) -> Vec<(Symbol, usize, usize, bool)> {
//     free.clone()
//         .into_iter()
//         .map(|mut x| {
//             if names.iter().find(|y| x.0 == y.0) != None {
//                 x.3 = true;
//                 x
//             } else {
//                 x
//             }
//         })
//         .collect()
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Chicken {
    Atom(String),
    Apply(Vec<Chicken>),
}

impl Chicken {
    fn to_string(&self) -> String {
        use Chicken::*;
        match self {
            Atom(atom) => atom.clone(),
            Apply(items) => {
                "(".to_owned() + &items.iter().map(|item|item.to_string()).collect::<Vec<String>>().join(" ") + ")"
            }
        }
    }
}

pub trait ToChicken {
    fn to_chicken(&self, env: &mut TranslationEnv) -> Result<Chicken>;
}

impl ToChicken for ABT<Term> {
    fn to_chicken(&self, env: &mut TranslationEnv) -> Result<Chicken> {
        match self {
            ABT::Var(symbol, usage) => Ok(Chicken::Atom(symbol.to_atom())),
            ABT::Tm(term) => term.to_chicken(env),
            ABT::Cycle(inner) => {
                let mut names = vec![];
                let (mut values, body) = unroll_cycle(inner, &mut names);
                let mut res = vec![Chicken::Atom("let".to_owned())];
                let mut bindings = vec![];
                // let mut buf = String::from("(let (");

                for i in 0..names.len() {
                    bindings.push(Chicken::Apply(vec![
                        Chicken::Atom(names[i].0.to_atom()),
                        values[i].to_chicken(env)?,
                    ]));
                    // match values[i].as_mut() {
                    //     ABT::Tm(Term::Lam(_body, free)) => {
                    //         // Filter out references to the items in the cycle
                    //         *free = filter_free_vbls(free, &names);
                    //     }
                    //     ABT::Tm(Term::Ann(inner, _)) => match inner.as_mut() {
                    //         ABT::Tm(Term::Lam(_body, free)) => {
                    //             // Filter out references to the items in the cycle
                    //             *free = filter_free_vbls(free, &names);
                    //         }
                    //         _ => {
                    //             // println!("NOT A TM {:?}", x);
                    //         }
                    //     },
                    //     _ => {
                    //         // println!("NOT A TM {:?}", x);
                    //     }
                    // };
                    // values[i].to_chicken(cmds, env)?;
                }
                return Ok(Chicken::Apply(vec![
                    Chicken::Atom("let".to_owned()),
                    Chicken::Apply(bindings),
                    body.to_chicken(env)?,
                ]))
                // Err(env::Error::NotImplemented("cycle".to_owned()))
                // names.reverse();
                // cmds.push(Chicken::Cycle(names));
                // body.to_chicken(cmds, env)?;
            }
            ABT::Abs(name, uses, body) => {
                // cmds.push(Chicken::PopAndName(name.clone(), *uses));
                // body.to_chicken(cmds, env)?;
                return Ok(Chicken::Apply(vec![Chicken::Atom(name.to_atom()), body.to_chicken(env)?]))
            }
        }
    }
}

fn unroll_cycle(
    inner: &ABT<Term>,
    names: &mut Vec<(Symbol, usize)>,
) -> (Vec<Box<ABT<Term>>>, Box<ABT<Term>>) {
    match inner {
        ABT::Abs(sym, uses, inner) => {
            names.push((sym.clone(), *uses));
            match &**inner {
                ABT::Tm(Term::LetRec(_, things, body)) => (things.clone(), body.clone()),
                _ => unroll_cycle(inner, names),
            }
        }
        _ => unreachable!("Cycle not abs"),
    }
}

pub struct TranslationEnv {
    pub env: env::Env,
    pub terms: HashMap<Hash, (Chicken, ABT<Type>)>,
    types: HashMap<Hash, TypeDecl>,
    pub anon_fns: Vec<(Hash, Chicken)>, // I think?
}

// impl Into<RuntimeEnv> for TranslationEnv {
//     fn into(self) -> RuntimeEnv {
//         RuntimeEnv {
//             terms: self.terms,
//             types: self.types,
//             anon_fns: self.anon_fns,
//         }
//     }
// }

// impl std::fmt::Debug for 

impl TranslationEnv {
    pub fn new(env: env::Env) -> Self {
        TranslationEnv {
            env,
            terms: HashMap::new(),
            types: HashMap::new(),
            anon_fns: vec![],
        }
    }

    pub fn to_string(&self, hash: &Hash) -> String {
        let mut res = String::from("(define (");
        res.push_str(&hash.to_string());
        res.push_str(")\n  ");
        let (term, typ) = self.terms.get(hash).unwrap();
        res.push_str(&term.to_string());
        res.push_str(")");
        return res
    }

    pub fn get_type(&mut self, hash: &Hash) -> TypeDecl {
        match self.types.get(hash) {
            Some(v) => v.clone(),
            None => {
                let res = self.env.load_type(&hash.to_string());
                self.types.insert(hash.clone(), res.clone());
                res
            }
        }
    }

    pub fn load(&mut self, hash: &Hash) -> Result<()> {
        if self.terms.contains_key(hash) {
            // Already loaded
            return Ok(());
        }
        // let mut cmds = ChickenEnv::new(hash.clone());
        self.terms.insert(
            hash.to_owned(),
            (
                Chicken::Atom("not-yet-evaluated".to_owned()),
                ABT::Tm(Type::Ref(Reference::Builtin("nvm".to_owned()))),
            ),
        );
        let (term, typ) = self.env.load(&hash.to_string())?;
        match term.to_chicken(self) {
            Ok(ch) => self.terms.insert(hash.to_owned(), (ch, typ)),
            Err(env::Error::NotImplemented(text)) => self.terms.insert(hash.to_owned(), (Chicken::Atom(text), typ)),
            Err(err) => {return Err(err)}
        };
        Ok(())
    }

    pub fn add_fn(&mut self, hash: Hash, contents: &ABT<Term>) -> Result<usize> {
        // let mut sub = ChickenEnv::new(hash.clone());
        let ch = contents.to_chicken(self)?;

        // resolve_marks(&mut sub.cmds);

        let idx = self.anon_fns.iter().position(|(_, cmds)| *cmds == ch);
        Ok(match idx {
            None => {
                let v = self.anon_fns.len();
                self.anon_fns.push((hash, ch));
                v
            }
            Some(idx) => idx,
        })
    }
}

// fn make_marks(cmds: &[Chicken]) -> HashMap<usize, usize> {
//     let mut marks = HashMap::new();
//     for i in 0..cmds.len() {
//         match &cmds[i] {
//             Chicken::Mark(m) => {
//                 marks.insert(*m, i);
//             }
//             _ => (),
//         }
//     }

//     marks
// }

// fn resolve_marks(cmds: &mut Vec<Chicken>) {
//     let marks = make_marks(cmds);
//     for cmd in cmds {
//         match cmd {
//             Chicken::Handle(mark) => {
//                 *mark = *marks.get(mark).unwrap();
//             }
//             Chicken::JumpTo(mark) => {
//                 *mark = *marks.get(mark).unwrap();
//             }
//             Chicken::IfAndPopStack(mark) => {
//                 *mark = *marks.get(mark).unwrap();
//             }
//             Chicken::If(mark) => {
//                 *mark = *marks.get(mark).unwrap();
//             }
//             _ => (),
//         }
//     }
// }

// pub struct ChickenEnv {
//     pub term: Hash,
//     pub cmds: Vec<Chicken>,
//     pub counter: usize,
// }

// impl ChickenEnv {
//     pub fn new(term: Hash) -> Self {
//         ChickenEnv {
//             term,
//             cmds: vec![],
//             counter: 0,
//         }
//     }

//     fn push(&mut self, chicken: Chicken) {
//         self.cmds.push(chicken)
//     }

//     fn mark(&mut self) -> usize {
//         self.counter += 1;
//         self.counter
//     }
// }

impl ToChicken for Term {
    fn to_chicken(&self, env: &mut TranslationEnv) -> Result<Chicken> {
        match self {
            // Term::Handle(handler, expr) => {
            //     Err(env::Error::NotImplemented("handle".to_owned()))
            // }
            Term::Ref(Reference::Builtin(name)) => Ok(Chicken::Atom(name.clone())),
            Term::Ref(Reference::DerivedId(Id(hash, _, _))) => {
                env.load(&hash)?;
                Ok(Chicken::Atom(hash.to_string()))
            }
            Term::App(one, two) => Ok(Chicken::Apply(vec![
                one.to_chicken(env)?,
                two.to_chicken(env)?,
            ])),
            Term::Int(num) => Ok(Chicken::Atom(num.to_string())),
            Term::Float(num) => Ok(Chicken::Atom(num.to_string())),
            Term::Nat(num) => Ok(Chicken::Atom(num.to_string())),
            Term::Boolean(num) => Ok(Chicken::Atom(num.to_string())),
            Term::Text(num) => Ok(Chicken::Atom(format!("{:?}", num))),
            Term::Char(num) => Ok(Chicken::Atom(num.to_string())),
            Term::If(cond, yes, no) => Ok(Chicken::Apply(vec![
                Chicken::Atom("if".to_owned()),
                cond.to_chicken(env)?,
                yes.to_chicken(env)?,
                no.to_chicken(env)?,
            ])),
            Term::And(one, two) => Ok(Chicken::Apply(vec![
                Chicken::Atom("and".to_owned()),
                one.to_chicken(env)?,
                two.to_chicken(env)?,
            ])),
            Term::Or(one, two) => Ok(Chicken::Apply(vec![
                Chicken::Atom("and".to_owned()),
                one.to_chicken(env)?,
                two.to_chicken(env)?,
            ])),
            // A constructor is a function that takes a number of
            // arguments, and returns a list with its name as the first item.
            Term::Constructor(Reference::Builtin(name), num) => {
                Ok(Chicken::Atom(format!("{}_{}", name, num)))
            },
            Term::Constructor(Reference::DerivedId(Id(hash, _, _)), num) => {
                Ok(Chicken::Atom(format!("{}_{}", hash.to_string(), num)))
            },
            _ => Ok(Chicken::Atom(format!("(not-implemented {:?})", format!("{:?}", self))))
            // _ => Err(env::Error::NotImplemented(format!("Term: {:?}", self)))
        }
    }
}

// impl ToChicken for Term {
//     fn to_chicken(&self, cmds: &mut ChickenEnv, env: &mut TranslationEnv) -> Result<()> {
//         match self {
//             Term::Handle(handler, expr) => {
//                 unreachable!();
//             }
//             Term::Ref(Reference::Builtin(_)) => cmds.push(Chicken::Value(self.clone().into())),
//             Term::Ref(Reference::DerivedId(Id(hash, _, _))) => {
//                 env.load(&hash)?;
//                 cmds.push(Chicken::Value(self.clone().into()))
//             }
//             Term::App(one, two) => {
//                 one.to_chicken(cmds, env)?;
//                 two.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::Call)
//             }
//             Term::Ann(term, _) => term.to_chicken(cmds, env)?,
//             Term::Sequence(terms) => {
//                 let ln = terms.len();
//                 for inner in terms {
//                     inner.to_chicken(cmds, env)?;
//                 }
//                 cmds.push(Chicken::Seq(ln))
//             }
//             Term::If(cond, yes, no) => {
//                 let no_tok = cmds.mark();
//                 let done_tok = cmds.mark();
//                 cond.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::If(no_tok));
//                 yes.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::JumpTo(done_tok));
//                 cmds.push(Chicken::Mark(no_tok));
//                 no.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::Mark(done_tok));
//             }
//             Term::And(a, b) => {
//                 let fail_tok = cmds.mark();
//                 let done_tok = cmds.mark();
//                 a.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::If(fail_tok));
//                 b.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::If(fail_tok));
//                 cmds.push(Chicken::Value(Value::Boolean(true)));
//                 cmds.push(Chicken::JumpTo(done_tok));
//                 cmds.push(Chicken::Mark(fail_tok));
//                 cmds.push(Chicken::Value(Value::Boolean(false)));
//                 cmds.push(Chicken::Mark(done_tok));
//             }
//             Term::Or(a, b) => {
//                 let good_tok = cmds.mark();
//                 let fail_tok = cmds.mark();
//                 let b_tok = cmds.mark();
//                 let done_tok = cmds.mark();
//                 a.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::If(b_tok));
//                 cmds.push(Chicken::JumpTo(good_tok));
//                 cmds.push(Chicken::Mark(b_tok));
//                 b.to_chicken(cmds, env)?;
//                 cmds.push(Chicken::If(fail_tok));

//                 cmds.push(Chicken::Mark(good_tok));
//                 cmds.push(Chicken::Value(Value::Boolean(true)));
//                 cmds.push(Chicken::JumpTo(done_tok));

//                 cmds.push(Chicken::Mark(fail_tok));
//                 cmds.push(Chicken::Value(Value::Boolean(false)));

//                 cmds.push(Chicken::Mark(done_tok));
//             }
//             Term::Let(_, v, body) => {
//                 v.to_chicken(cmds, env)?;
//                 body.to_chicken(cmds, env)?;
//             }
//             Term::Match(item, arms) => {
//                 let done_tok = cmds.mark();
//                 item.to_chicken(cmds, env)?;
//                 let mut next_tok = cmds.mark();
//                 for MatchCase(pattern, cond, body) in arms {
//                     match cond {
//                         None => {
//                             cmds.push(Chicken::PatternMatch(pattern.clone(), false));
//                             cmds.push(Chicken::If(next_tok));
//                         }
//                         Some(cond) => {
//                             // TODO should I have an ID with these,
//                             // to catch me of I pop the stack too much?
//                             cmds.push(Chicken::MarkStack);
//                             cmds.push(Chicken::PatternMatch(pattern.clone(), true));
//                             cmds.push(Chicken::IfAndPopStack(next_tok));
//                             cond.to_chicken(cmds, env)?;
//                             cmds.push(Chicken::IfAndPopStack(next_tok));
//                             cmds.push(Chicken::ClearStackMark);
//                         }
//                     }

//                     body.to_chicken(cmds, env)?;
//                     cmds.push(Chicken::JumpTo(done_tok));

//                     cmds.push(Chicken::Mark(next_tok));
//                     next_tok = cmds.mark();
//                 }
//                 cmds.push(Chicken::PatternMatchFail);
//                 cmds.push(Chicken::Mark(done_tok));
//                 cmds.push(Chicken::PopUpOne);
//             }
//             Term::Lam(contents, free_vbls) => {
//                 let v = env.add_fn(cmds.term.clone(), &**contents)?;
//                 cmds.push(Chicken::Fn(v, free_vbls.clone()));
//             }
//             Term::Request(Reference::Builtin(name), _) => {
//                 unimplemented!("Builtin Effect! I dont know the arity: {}", name);
//             }
//             Term::Request(Reference::DerivedId(id), number) => {
//                 let t = env.get_type(&id.0);
//                 match t {
//                     TypeDecl::Effect(DataDecl { constructors, .. }) => {
//                         let args = calc_args(&constructors[*number].1);
//                         cmds.push(Chicken::Value(Value::RequestWithArgs(
//                             Reference::DerivedId(id.clone()),
//                             *number,
//                             args,
//                             // ok, this is useless allocation if there are no
//                             vec![],
//                         )))
//                     }
//                     _ => unimplemented!("ok"),
//                 }
//             }

//             _ => cmds.push(Chicken::Value(self.clone().into())),
//         };
//         Ok(())
//     }
// }

// fn calc_args(t: &ABT<Type>) -> usize {
//     match t {
//         ABT::Tm(t) => match t {
//             Type::Effect(_, _) => 0,
//             Type::Arrow(_, inner) => 1 + calc_args(&*inner),
//             Type::Forall(inner) => calc_args(inner),
//             _ => unimplemented!("Unexpected element of a request {:?}", t),
//         },
//         ABT::Abs(_, _, inner) => calc_args(inner),
//         ABT::Cycle(inner) => calc_args(inner),
//         _ => unimplemented!("Unexpected ABT"),
//     }
// }