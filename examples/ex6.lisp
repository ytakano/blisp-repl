(export callback (x y z)
    (IO (-> (Int Int Int) (Option Int)))
    (call-rust x y z))