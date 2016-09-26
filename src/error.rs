#[macro_export]
macro_rules! def_error {
    ( $($x:ident: $y:expr,)* ) => {

        #[derive(Debug)]
        pub enum Error {
            $($x,)*
        }

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                error::Error::description(self).fmt(f)
            }
        }

        impl error::Error for Error {
            fn description(&self) -> &str {
                match *self {
                    $(Error::$x => $y,)*
                }
            }
        }
    }
}
