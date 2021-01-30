(import (chicken bitwise))
(import (chicken condition))
(require-extension utf8)

(define true #t)
(define false #f)

(define (term-link v) (list 'term-link v))
(define (type-link v) (list 'type-link v))

(define (print-processing name)
    ;; Uncomment this line to debug terms that are failing to process
    (print "Evaluating " name)
    '())

(define (check v name)
    (if (not v)
    (begin
        (print "Test failed! " name)
        (print "Got " v)
        ; (print "Test failure")
        )
    (print "✅ passed " name)
        )
    )

(define (result-is-good result)
    (and (list? result)
        (= (length result) 2)
        (equal? (car result) 'vmc06s4f236sps61vqv35g7ridnae03uetth98aocort1825stbv7m6ncfca2j0gcane47c8db2rjtd2o6kch2lr7v2gst895pcs0m0_1)
    )
)

(define (check-results v name)
    (map
        (lambda (result)
            (if (not (result-is-good result))
            (begin
                (print "Test failed " name " " result)
                (abort "Test failed")
            )))
        (vector->list v)
    )
    (print "✅ passed " name)
    '()
)

(define (f2c22r2a1sche28mn07brk1j45kp1bam3tr4k2j0un2hi1g7rbrud3f5mes2defqo1tpd9j38pqpg2f0efl3no0ede5ocl2am4bonm0 a)
    (lambda (b) (not (equal? a b)))
)

; GUID

(define (rc29vdqe019p56kupcgkg07fkib86r3oooatbmsgfbdsgpmjhsh00l307iuts3r973q5etb61vbjkes42b6adb3mkorusvmudiuorno_0 id)
    (list 'rc29vdqe019p56kupcgkg07fkib86r3oooatbmsgfbdsgpmjhsh00l307iuts3r973q5etb61vbjkes42b6adb3mkorusvmudiuorno_0 id))

(define (Bytes.fromList bytes) bytes) ; yolo

; base.Map

(define (7di5ureqgi60ue42886240kbovfhko0fg85rp2thpkl8af699upsl0os1btk27te1cjdmuerad5oi9bdd04me6mjh2m25djbj236fbo_0 k)
    (lambda (v) (list '7di5ureqgi60ue42886240kbovfhko0fg85rp2thpkl8af699upsl0os1btk27te1cjdmuerad5oi9bdd04me6mjh2m25djbj236fbo_0 k v)))

; Cons/Nil

(define (onbcm0qctbnuctpm57tkc5p16b8gfke8thjf19p4r4laokji0b606rd0frnhj103qb90lve3fohkoc1eda70491hot656s1m6kk3cn0_0 one)
    (lambda (two) (list 'onbcm0qctbnuctpm57tkc5p16b8gfke8thjf19p4r4laokji0b606rd0frnhj103qb90lve3fohkoc1eda70491hot656s1m6kk3cn0_0 one two)))

(define 568rsi7o3ghq8mmbea2sf8msdk20ohasob5s2rvjtqg2lr0vs39l1hm98urrjemsr3vo3fa52pibqu0maluq7g8sfg3h5f5re6vitj8_0
    '568rsi7o3ghq8mmbea2sf8msdk20ohasob5s2rvjtqg2lr0vs39l1hm98urrjemsr3vo3fa52pibqu0maluq7g8sfg3h5f5re6vitj8_0)

(define Cons onbcm0qctbnuctpm57tkc5p16b8gfke8thjf19p4r4laokji0b606rd0frnhj103qb90lve3fohkoc1eda70491hot656s1m6kk3cn0_0 )
(define Nil 568rsi7o3ghq8mmbea2sf8msdk20ohasob5s2rvjtqg2lr0vs39l1hm98urrjemsr3vo3fa52pibqu0maluq7g8sfg3h5f5re6vitj8_0)


; Some/None
(define 5isltsdct9fhcrvud9gju8u0l9g0k9d3lelkksea3a8jdgs1uqrs5mm9p7bajj84gg8l9c9jgv9honakghmkb28fucoeb2p4v9ukmu8_0
    '5isltsdct9fhcrvud9gju8u0l9g0k9d3lelkksea3a8jdgs1uqrs5mm9p7bajj84gg8l9c9jgv9honakghmkb28fucoeb2p4v9ukmu8_0)

(define (5isltsdct9fhcrvud9gju8u0l9g0k9d3lelkksea3a8jdgs1uqrs5mm9p7bajj84gg8l9c9jgv9honakghmkb28fucoeb2p4v9ukmu8_1 arg)
    (list '5isltsdct9fhcrvud9gju8u0l9g0k9d3lelkksea3a8jdgs1uqrs5mm9p7bajj84gg8l9c9jgv9honakghmkb28fucoeb2p4v9ukmu8_1 arg))

(define None 5isltsdct9fhcrvud9gju8u0l9g0k9d3lelkksea3a8jdgs1uqrs5mm9p7bajj84gg8l9c9jgv9honakghmkb28fucoeb2p4v9ukmu8_0)
(define Some 5isltsdct9fhcrvud9gju8u0l9g0k9d3lelkksea3a8jdgs1uqrs5mm9p7bajj84gg8l9c9jgv9honakghmkb28fucoeb2p4v9ukmu8_1)



(define (m7uplgfko92kqdmm6u898j5h4n86587f44u7fq1vjcad1f68n35r8j2mdfdbjta5hq9o699dgn2aphteditp30g34hsh3gru68593j0 a)
    (lambda (b) (- a b)))

(define (Nat.drop a)
    (lambda (b)
        (max 0 (- a b))))

(define (Nat.+ a)
    (lambda (b)
        (if (not (number? a))
        (begin
        (print "A is " a " " (symbol? a) " " (string? a))
        (abort "a is not a number")
        )
        )
        (natLoop (+ a b))))

(define (natLoop num)
    (if (> num maxNat) (- num 1 maxNat) num))

(define (intLoop num)
    (if (> num maxInt)
        (+ minInt (- num 1 maxInt))
        (if (< num minInt)
            (+ maxInt (- num -1 minInt))
            num
        )
    ))


(define maxNat 18446744073709552000)
(define minInt -18446744073709552000)
(define maxInt 18446744073709552000)

(define s9h25aadei68iscfiu60eldfhe9uvh0pk3knd9m965gqlejvc5jlcqs9gfcgpgvfv85n2pefvee4ca2n7mepcoqamou73g7ilscf450
    maxNat)

(define p9og3s2h41natoslfjoi1do0omp82s4jiethebfd4j5p99ltbdmcua2egbiehs9tq9k65744cvugibiqdkgip21t7se4e8faktnl3k0
    minInt)

(define d75vubeoep5o8ph72v0v9qdm36n17up0d7bsbdckjapcs7k9g1kv5mnbpp3444u8fmvo2h3benmk7o3sd09g1lkrrvk4q93vv8u2n3g
    maxInt)

(define (Nat.xor a)
    (lambda (b) (bitwise-xor a b)))

(define (Nat.pow a)
    (lambda (b) (expt a b)))


(define (Text.!= a) (lambda (b) (not (equal? a b))))
(define (Float.* a) (lambda (b) (* a b)))
(define (Float./ a) (lambda (b) (/ a (exact->inexact b))))
(define (Float.- a) (lambda (b) (- a b)))
(define (Float.+ a) (lambda (b) (+ a b)))
(define Boolean.not not)

        
(define (Int.- a)
    (lambda (b)
        (intLoop (- a b))))

        
(define (Int.+ a)
    (lambda (b)
        (intLoop (+ a b))))


        
(define (Nat.* a)
    (lambda (b)
        (* a b)))

        
(define (Nat.or a)
    (lambda (b)
        (bitwise-ior a b)))

        
(define (Nat.and a)
    (lambda (b)
        (bitwise-and a b)))

(define (Nat.shiftLeft a)
    (lambda (b)
        (arithmetic-shift a b)))

        
(define (Nat.shiftRight a)
    (lambda (b)
        (arithmetic-shift a (- b))))

(define (Int.toText n) 
    (if (>= n 0)
        (string-append "+" (number->string n))
        (number->string n)
        ))
(define Int.isEven even?)
(define Int.isOdd odd?)

(define (Int.pow a) (lambda (b) (expt a b)))
(define (Int.mod a) (lambda (b) (modulo a b)))
(define (Int.complement a) (bitwise-not a))
(define (Int.or a) (lambda (b) (bitwise-ior a b)))
(define (Int.and a) (lambda (b) (bitwise-and a b)))
(define (Int.xor a) (lambda (b) (bitwise-xor a b)))
(define (Int.increment a) (+ a 1))
(define (Int.decrement a) (- a 1))
(define (Int./ a) (lambda (b) (/ a b)))
(define (Int.* a) (lambda (b) (* a b)))
(define (Int.negate a) (- a))

(define Nat.toText number->string)
(define (Nat.toInt x) x)
(define (Nat.sub a) (lambda (b) (- a b)))
(define Nat.isEven even?)
(define Nat.isOdd odd?)
(define (Nat.mod a) (lambda (b) (modulo a b)))
(define (Nat.increment a) (natLoop (+ a 1)))
(define (Nat.decrement a) (max 0 (- a 1)))
(define (Nat./ a) (lambda (b) (floor (/ a b))))
(define (Nat.* a) (lambda (b) (* a b)))

(define (Int.shiftLeft a)
    (lambda (b)
        (arithmetic-shift a b)))

(define (Int.shiftRight a)
    (lambda (b)
        (arithmetic-shift a (- b))))


(import srfi-67)

(import srfi-128)
(define default-comparator (make-default-comparator))

(define (Universal.> a) (lambda (b) (>? default-comparator a b)))
(define (Universal.>= a) (lambda (b) (>=? default-comparator a b)))
(define (Universal.<= a) (lambda (b) (<=? default-comparator a b)))
(define (Universal.< a) (lambda (b) (<? default-comparator a b)))
(define (Universal.== a) (lambda (b) (=? default-comparator a b)))
(define (Universal.compare a) (lambda (b) (comparator-if<=> default-comparator a b -1 0 1)))

; --- lists ---

;; using vectors
(import srfi-133)
(define List.size vector-length)
(define (List.cons item) (lambda (vec)
    (let ((dest (make-vector (+ 1 (vector-length vec)))))
        (vector-copy! dest 1 vec)
        (vector-set! dest 0 item)
        dest)))
(define (List.snoc vec) (lambda (item)
    (let ((dest (make-vector (+ 1 (vector-length vec)))))
        (vector-copy! dest 0 vec)
        (vector-set! dest (vector-length vec) item)
        dest)))
(define (List.++ a) (lambda (b) (vector-append a b))) 
(define (List.drop count) (lambda (vec)
    (let ((count (min (vector-length vec) count)))
        (let ((ln (- (vector-length vec) count)))
            (let ((dest (make-vector ln)))
                (vector-copy! dest 0 vec count)
                dest)))))
(define (List.at a) (lambda (b)
    (if (< a (vector-length b))
        (Some (vector-ref b a))
        None)))
(define (List.take ln) (lambda (vec)
    (let ((ln_ (min ln (vector-length vec))))
        (let ((dest (make-vector ln_)))
            (vector-copy! dest 0 vec 0 ln_)
            dest))))

;; using linked lists
; (define List.size length)
; (define List.cons cons)
; (define (List.++ a) (lambda (b) (append a b))) 
; (define (List.drop a) (lambda (b) (list-tail b a)))
; (define (List.at a) (lambda (b) (list-ref b a)))

; --- text stdlib ---

(define (Text.++ a) (lambda (b) (string-append a b)))
(define Text.size string-length)
(define (Text.take count) (lambda (str)
    (let ((count (min count (string-length str))))
        (substring str 0 count))))
(define (Text.drop count) (lambda (str)
    (let ((count (min count (string-length str))))
        (substring str count))))

; --- abilties ---

(define stack '())

(define name "wip")

(define (throw-effect k effect)
    (if (or (not (list? effect))
        (not (procedure? k))
        (not (symbol? (car effect)))
        )
        (begin
        (print "Invalid input to throw-effect " k " " effect)
        (abort "Invalid throw effect"))
    (if (eq? '() stack)
        (abort "no handler for effect")
        (let* ((handler (car stack)))
            (set! stack (cdr stack))
            ;(print "Calling " name)
            (handler (cons 'effect (cons k effect)))
        ))
    )
)

(define (throw-pure value)
    (let* ((handler (car stack))  )
        (set! stack (cdr stack))
        ;(print "Calling " name)
        (handler (list 'pure value))
    )
)

(define (rethrow-effect eff)
    (let ((k (cadr eff))
          (ef (cddr eff)))
        (throw-effect k ef)))

(define (add-handler handler)
    ;(print "add handler " name " " handler " " stack)
    (set! stack (cons handler stack))
    ;(print "now " stack)
)


(define (handle-ability inner handler)
    (handler (call/cc (lambda (k)
        ;(print "adding handler " name)
        (add-handler k) ; TODO we'll have to pop at some point?
        (let ((value (inner)))
            ;(print "Got value " name " " value)
            (throw-pure value)
        )
    )))
)
