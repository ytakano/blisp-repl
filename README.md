# BLisp's repl

## Build

```
$ cargo build
```

## Run (and Build if necessary)

```
$ cargo run ./examples/ex1.lisp
```

## Examples

### Example 1

Function definition.

```
$ cargo run examples/ex1
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/blisp-repl examples/ex1.blisp`
(export factorial (n) (Pure (-> (Int) Int))
    (factorial' n 1))

(defun factorial' (n total) (Pure (-> (Int Int) Int))
    (if (<= n 0)
        total
        (factorial' (- n 1) (* n total))))
CTRL-D to exit
>> (factorial 10)
3628800
>> (factorial 4)
24
>> (factorial 200)
788657867364790503552363213932185062295135977687173263294742533244359449963403342920304284011984623904177212138919638830257642790242637105061926624952829931113462857270763317237396988943922445621451664240254033291864131227428294853277524242407573903240321257405579568660226031904170324062351700858796178922222789623703897374720000000000000000000000000000000000000000000000000
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
'(2 3)
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

### Example 6

Call Rust's function.

```
$ cargo run ./examples/ex6.lisp
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/blisp-repl ./examples/ex6.lisp`
(export callback (x y z)
    (IO (-> (Int Int Int) (Option Int)))
    (call-rust x y z))
CTRL-D to exit
>> (callback 10 20 30)
Rust's function is called: n = 6000
(Some 6000)
```
