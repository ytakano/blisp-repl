# BLisp's repl

## Build

```
$ cargo build
```

## Run (and Build if necessary)

```
$ cargo run ./examples/ex1.blisp
```

## Examples

### Example 1

Function definition.

```
$ cargo run examples/ex1
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/blisp-repl examples/ex1.blisp`
(export factorial (n) (Pure (-> (Int) Int))
    (if (<= n 0)
        1
        (* n (factorial (- n 1)))))
CTRL-D to exit
>> (factorial 10)
3628800
>> (factorial 4)
24
>> (+ 1 2)
3
```

### Example 2

Local function definition.

```
$ cargo run examples/ex2.blisp
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/blisp-repl examples/ex2.blisp`
(export lambda-test (f) (Pure (-> ((Pure (-> (Int) Int))) Int))
    (mul2 (f 2)))

(defun mul2 (x) (Pure (-> (Int) Int))
    (* 2 x))
CTRL-D to exit
>> (lambda-test (lambda (x) (+ x 80)))
164
>> (lambda-test (lambda (x) (* x x)))
8
```

### Example 3

Algebraic data type and pattern match.

```
$ cargo run examples/ex3.blisp
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/blisp-repl examples/ex3.blisp`
(export label-test (x) (Pure (-> ((Option Int)) Int))
    (match x
        ((Some x) x)
        (None 0)))
CTRL-D to exit
>> (label-test (Some 10))
10
>> (label-test None)
0
>>
CTRL-D
```

### Example 4

List and pattern match.

```
$ cargo run examples/ex4.blisp
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/blisp-repl examples/ex4.blisp`
(export last (x) (Pure (-> ('(Int)) (Option Int)))
    (match x
        (Nil None)
        ((Cons n Nil) (Some n))
        ((Cons _ l) (last l))))

CTRL-D to exit
>> (cdr '(1 2 3))
(Cons 2 (Cons 3 Nil))
>> (cdr '())
Nil
>> (car '(3 5 2))
(Some 3)
>> (car '())
None
>> (last '(5 7 3))
(Some 3)
>> (last '())
None
```

### Example 5

Tuple and pattern match.

```
$ cargo run examples/ex5.blisp
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/blisp-repl examples/ex5.blisp`
(export first (x) (Pure (-> ([Int Bool]) Int))
    (match x
        ([n _] n)))

(export second (x) (Pure (-> ([Int Bool]) Bool))
    (match x
        ([_ n] n)))

(export none (x) (Pure (-> ([]) Bool))
    (match x
        ([] true)))
CTRL-D to exit
>> (none [])
true
>> (second [13 false])
false
>> (first [63 true])
63
```
