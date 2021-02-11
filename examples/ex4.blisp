(export last (x) (Pure (-> ('(Int)) (Option Int)))
    (match x
        (Nil None)
        ((Cons n Nil) (Some n))
        ((Cons _ l) (last l))))
