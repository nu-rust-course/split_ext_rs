#[macro_export]
macro_rules! if_opt {
    { $cond:expr, $body:expr }
        =>
    {
        if $cond {
            Some($body)
        } else {
            None
        }
    };
}

