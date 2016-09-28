#[macro_export]
macro_rules! def_error {
    ( $($x:ident: $y:expr,)* ) => {
        #[derive(Debug)]
        pub enum Error {
            $($x,)*
        }

        impl ::std::fmt::Display for Error {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
                   -> ::std::fmt::Result {
                ::std::error::Error::description(self).fmt(f)
            }
        }

        impl ::std::error::Error for Error {
            fn description(&self) -> &str {
                match *self {
                    $(Error::$x => $y,)*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! allow_once {
    () => {
        static mut initialized: bool = false;
        unsafe {
            if initialized {
                return Err(Error::Initialized);
            } else {
                initialized = true;
            }
        }
    }
}
