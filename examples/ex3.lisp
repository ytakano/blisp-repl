(export label-test (x) (Pure (-> ((Option Int)) Int))
    (match x
        ((Some x) x)
        (None 0)))