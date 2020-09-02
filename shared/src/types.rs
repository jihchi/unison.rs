use super::base32hex;
use im::Vector;
use serde_derive::{Deserialize, Serialize};
use std::cmp::{PartialEq, PartialOrd};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, PartialOrd, Hash, Eq)]
pub struct Symbol {
    pub num: usize,
    pub text: String,
    // TODO make this an optional, so that I can make partialeq make sense
    // before we've uniqued.
    pub unique: usize,
}

impl Symbol {
    pub fn with_unique(&self, unique: usize) -> Self {
        Symbol {
            num: self.num,
            text: self.text.clone(),
            unique,
        }
    }
    // For when unique is uninitialized
    pub fn pre_eq(&self, other: &Self) -> bool {
        self.num == other.num && self.text == other.text
    }
}

impl std::cmp::PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.unique == other.unique
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("🔣{}/{}", self.text, self.unique))
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    std::cmp::Eq,
    std::cmp::PartialEq,
    std::hash::Hash,
    PartialOrd,
)]
pub enum ConstructorType {
    Data,
    Effect,
}

#[derive(
    Serialize, Deserialize, Clone, std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash, PartialOrd,
)]
pub enum Reference {
    Builtin(String),
    DerivedId(Id),
}

impl std::fmt::Debug for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Reference::Builtin(name) => f.write_str(name),
            Reference::DerivedId(id) => f.write_str(&format!("{:?}", id.0)),
        }
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str("#")?;
        f.write_str(&base32hex::encode(&self.0)[0..10])
    }
}

impl Reference {
    pub fn from_hash(hash: &str) -> Self {
        Reference::DerivedId(Id(Hash::from_string(hash), 0, 1))
    }
}

#[derive(
    Serialize, Deserialize, Clone, std::cmp::Eq, std::cmp::PartialEq, std::hash::Hash, PartialOrd,
)]
pub struct Hash(pub Vec<u8>);

impl Hash {
    pub fn from_string(hash: &str) -> Self {
        if hash == "<eval>" {
            return Hash(vec![]);
        }
        let data = base32hex::decode(hash);
        Hash(data)
    }
    pub fn to_string(&self) -> String {
        if self.0.len() == 0 {
            return "<eval>".to_owned();
        }
        let mut m = base32hex::encode(&self.0);
        m.pop();
        m
    }
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    std::cmp::Eq,
    std::cmp::PartialEq,
    std::hash::Hash,
    PartialOrd,
)]
pub struct Id(pub Hash, pub usize, pub usize);

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    std::cmp::Eq,
    std::cmp::PartialEq,
    std::hash::Hash,
    PartialOrd,
)]
pub enum Referent {
    Ref(Reference),
    Con(Reference, usize, ConstructorType),
}

// impl Referent {
//     fn reference(&self) -> &Reference {
//         match self {
//             Referent::Ref(r) => r,
//             Referent::Con(r, _, _) => r,
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct MatchCase(pub Pattern, pub Option<Box<ABT<Term>>>, pub Box<ABT<Term>>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Kind {
    Star,
    Arrow(Box<Kind>, Box<Kind>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Pattern {
    Unbound,
    Var,
    Boolean(bool),
    Int(i64),
    Nat(u64),
    Float(f64),
    Text(String),
    Char(char),
    Constructor(Reference, usize, Vec<Pattern>),
    As(Box<Pattern>),
    EffectPure(Box<Pattern>),
    EffectBind(Reference, usize, Vec<Pattern>, Box<Pattern>),
    SequenceLiteral(Vec<Pattern>),
    SequenceOp(Box<Pattern>, SeqOp, Box<Pattern>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum SeqOp {
    Cons,
    Snoc,
    Concat,
}

// Base functor for types in the Unison language
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Ref(Reference),
    Arrow(Box<ABT<Type>>, Box<ABT<Type>>),
    Ann(Box<ABT<Type>>, Kind),
    App(Box<ABT<Type>>, Box<ABT<Type>>),
    Effect(Box<ABT<Type>>, Box<ABT<Type>>),
    Effects(Vec<ABT<Type>>),
    Forall(Box<ABT<Type>>),
    //  binder like ∀, used to introduce variables that are
    //  bound by outer type signatures, to support scoped type
    //  variables
    IntroOuter(Box<ABT<Type>>),
}

// Runtime values
#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Nat(u64),
    Float(f64),
    Boolean(bool),
    Text(String),
    Bytes(Vec<u64>),
    Char(char),
    Ref(Reference),
    CycleBlank(usize),

    CycleFnBody(
        usize,
        // bindings for this one, with CycleBlanks
        // where mutuals would be
        Vec<(Symbol, usize, Arc<Value>)>,
        // mutuals!
        Vec<(
            Symbol, // the ID identified in CycleBlank
            usize,  // usage number? maybe not relevant
            usize,  // the fn ID
            // the bindings for this one, with CycleBlanks
            // where mutuals would be
            Vec<(Symbol, usize, Arc<Value>)>,
        )>,
        // Vec<(Symbol, usize, usize, Vec<usize>)>,
    ),
    PartialFnBody(usize, Vec<(Symbol, usize, Arc<Value>)>),
    PartialNativeApp(String, Vec<Arc<Value>>),
    PartialConstructor(Reference, usize, Vector<Arc<Value>>),

    Continuation(usize, Vec<super::frame::Frame>),
    Constructor(Reference, usize),
    Request(Reference, usize),
    RequestPure(Arc<Value>),
    RequestWithArgs(Reference, usize, usize, Vec<Arc<Value>>),
    RequestWithContinuation(
        Reference,
        usize,
        Vec<Arc<Value>>,
        usize,
        Vec<super::frame::Frame>,
        usize,
    ),

    Sequence(Vector<Arc<Value>>),
    TermLink(Referent),
    TypeLink(Reference),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Term {
    Int(i64),
    Nat(u64),
    Float(f64),
    Boolean(bool),
    Text(String),
    Bytes(Vec<u64>),
    Char(char),
    Blank,
    Ref(Reference),
    Constructor(Reference, usize),
    Request(Reference, usize),
    Handle(Box<ABT<Term>>, Box<ABT<Term>>),
    App(Box<ABT<Term>>, Box<ABT<Term>>),
    Ann(Box<ABT<Term>>, ABT<Type>),
    Sequence(Vec<Box<ABT<Term>>>),
    If(Box<ABT<Term>>, Box<ABT<Term>>, Box<ABT<Term>>),
    And(Box<ABT<Term>>, Box<ABT<Term>>),
    Or(Box<ABT<Term>>, Box<ABT<Term>>),
    // the bool is whether this is a cycle vbl or not?
    Lam(
        Box<ABT<Term>>,
        Vec<(
            Symbol,
            usize, // external usage number
            usize, // internal usage count
            bool,
        )>,
    ),
    //   -- Note: let rec blocks have an outer ABT.Cycle which introduces as many
    //   -- variables as there are bindings
    LetRec(bool, Vec<Box<ABT<Term>>>, Box<ABT<Term>>),
    //   -- Note: first parameter is the binding, second is the expression which may refer
    //   -- to this let bound variable. Constructed as `Let b (abs v e)`
    Let(bool, Box<ABT<Term>>, Box<ABT<Term>>),
    //   -- Pattern matching / eliminating data types, example:
    //   --  case x of
    //   --    Just n -> rhs1
    //   --    Nothing -> rhs2
    //   --
    //   -- translates to
    //   --
    //   --   Match x
    //   --     [ (Constructor 0 [Var], ABT.abs n rhs1)
    //   --     , (Constructor 1 [], rhs2) ]
    Match(Box<ABT<Term>>, Vec<MatchCase>),
    TermLink(Referent),
    TypeLink(Reference),
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Term::*;
        match self {
            Int(i) => f.write_fmt(format_args!("{}", i)),
            Nat(i) => f.write_fmt(format_args!("{}", i)),
            Float(i) => f.write_fmt(format_args!("{}", i)),
            Boolean(i) => f.write_fmt(format_args!("{}", i)),
            Text(i) => f.write_fmt(format_args!("{:?}", i)),
            Bytes(i) => f.write_fmt(format_args!("{:?}", i)),
            Char(i) => f.write_fmt(format_args!("{:?}", i)),
            Blank => f.write_str("<blank>"),
            Ref(i) => f.write_fmt(format_args!("{:?}", i)),
            Constructor(i, n) => f.write_fmt(format_args!("[constructor]{:?}#{}", i, n)),
            Request(i, n) => f.write_fmt(format_args!("[request]{:?}#{}", i, n)),
            Handle(i, n) => f.write_fmt(format_args!("handle {:?} with {:?}", i, n)),
            App(i, n) => f.write_fmt(format_args!("{:?} <app> {:?}", i, n)),
            Ann(i, n) => f.write_fmt(format_args!("t- {:?} :: {:?} -t", i, n)),
            Sequence(i) => f.write_fmt(format_args!("{:?}", i)),
            If(i, a, b) => f.write_fmt(format_args!("if {:?} then\n{:?}\nelse\n{:?}", i, a, b)),
            And(a, b) => f.write_fmt(format_args!("{:?} && {:?}", a, b)),
            Or(a, b) => f.write_fmt(format_args!("{:?} || {:?}", a, b)),
            Lam(a, _free) => f.write_fmt(format_args!("-> {:?}", a)),
            LetRec(_, a, b) => f.write_fmt(format_args!("let(rec)\n{:?}\nin {:?}", a, b)),
            Let(_, a, b) => f.write_fmt(format_args!("let {:?} in {:?}", a, b)),
            Match(a, b) => f.write_fmt(format_args!("match {:?} with {:?}", a, b)),
            TermLink(a) => f.write_fmt(format_args!("termLink {:?}", a)),
            TypeLink(a) => f.write_fmt(format_args!("typeLink {:?}", a)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum ABT<Content> {
    Var(Symbol, usize), // usage number
    Cycle(Box<ABT<Content>>),
    // number of usages expected
    Abs(Symbol, usize, Box<ABT<Content>>),
    Tm(Content),
}

impl<Inner: std::fmt::Debug> std::fmt::Debug for ABT<Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ABT::Tm(i) => f.write_fmt(format_args!("{:?}", i)),
            ABT::Var(i, u) => f.write_fmt(format_args!("〰️{} (#{})", i.text, u)),
            ABT::Cycle(i) => f.write_fmt(format_args!("🚲({:?})", i)),
            ABT::Abs(s, u, i) => {
                f.write_fmt(format_args!("|{}/{} #{}|({:?})", s.text, s.unique, u, i))
            }
        }
    }
}

#[derive(
    Serialize, Deserialize, Debug, Clone, std::cmp::Eq, std::hash::Hash, std::cmp::PartialEq,
)]
pub struct NameSegment {
    pub text: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Modifier {
    Structural,
    Unique(String),
    Opaque,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TypeDecl {
    Effect(DataDecl),
    Data(DataDecl),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataDecl {
    pub modifier: Modifier,
    pub bound: Vec<Symbol>,
    pub constructors: Vec<(Symbol, ABT<Type>)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Star<K: std::hash::Hash + std::cmp::Eq + Clone, V> {
    pub fact: std::collections::HashSet<K>,
    pub d1: HashMap<K, V>,
    pub d2: HashMap<K, Reference>,
    pub d3: HashMap<K, (Reference, Reference)>,
}

impl Into<Value> for Term {
    fn into(self) -> Value {
        match self {
            Term::Int(i) => Value::Int(i),
            Term::Nat(a) => Value::Nat(a),
            Term::Float(a) => Value::Float(a),
            Term::Boolean(a) => Value::Boolean(a),
            Term::Text(a) => Value::Text(a),
            Term::Bytes(a) => Value::Bytes(a),
            Term::Char(a) => Value::Char(a),
            Term::Ref(a) => Value::Ref(a),
            Term::Constructor(a, b) => Value::Constructor(a, b),
            Term::Request(a, b) => Value::Request(a, b),
            Term::TermLink(a) => Value::TermLink(a),
            Term::TypeLink(a) => Value::TypeLink(a),
            _ => unreachable!("Cannot convert to a value"),
        }
    }
}

// So I think we have a scope and a stack?
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IR {
    Handle(usize), // indicate that there's a handler at `usize`
    HandlePure,
    // This means "grab the function identified by usize"
    // but maybe this should be a Term?
    // I mean I should make a different `Value` deal, but not
    // just this moment
    // The bool is whether this is a cycle vbl
    Fn(
        usize,
        Vec<(
            Symbol,
            usize, // usage number at fn creation site
            usize, // number of usages to expect within the FN. NOTE we can get rid of this if we switch to just "is this the last" calculation.
            bool,
        )>,
    ),
    // Builtin(String),
    Cycle(Vec<(Symbol, usize)>),
    // CycleFn(usize, Vec<(Symbol, usize)>),
    // Push this value onto the stack
    Value(Value),
    Pop,
    // lookup the symbol, and push it onto the stack
    PushSym(Symbol, usize),
    // pop the top value off the stack and give it a name
    PopAndName(Symbol, usize),
    // pop the top two values off the stack, call the first with the second
    Call,
    // Swap the top two values
    Swap,
    // Pop the top N values from the stack, assemble into a seq
    Seq(usize),
    JumpTo(usize),
    Mark(usize),
    // pop the last value off the stack;
    // if it's true, advance.
    /// otherwise, jump to the given mark
    If(usize),
    // If2(usize, usize),
    // hmm I might want to short-circut?
    // And,
    // Or,
    // Dup, // duplicate the top item - might not need it
    PopUpOne,
    // Match the given pattern.
    // If the "has_where" flag is true, bound variables
    // will be pushed onto the stack twice
    PatternMatch(Pattern, bool),
    PatternMatchFail,
    MarkStack,
    ClearStackMark,
    // if false, then pop up to the stack mark.
    // if true, the following code will bind those vbls, its fine.
    IfAndPopStack(usize),
}

#[derive(Serialize, Deserialize)]
pub struct RuntimeEnv {
    pub terms: HashMap<Hash, (Vec<IR>, ABT<Type>)>,
    pub types: HashMap<Hash, TypeDecl>,
    pub anon_fns: Vec<(Hash, Vec<IR>)>, // I think?
}

impl RuntimeEnv {
    pub fn new() -> RuntimeEnv {
        RuntimeEnv {
            terms: HashMap::new(),
            types: HashMap::new(),
            anon_fns: vec![],
        }
    }
}
