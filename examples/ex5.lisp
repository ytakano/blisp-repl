(export first (x) (Pure (-> ([Int Bool]) Int))
    (match x
        ([n _] n)))

(export second (x) (Pure (-> ([Int Bool]) Bool))
    (match x
        ([_ n] n)))

(export none (x) (Pure (-> ([]) Bool))
    (match x
        ([] true)))