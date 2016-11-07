#[macro_export]
macro_rules! def_error {
    ( $($x:ident: $y:expr,)* ) => {
        #[derive(Debug)]
        pub enum Error {
            $($x,)*
        }

        pub type Result<T> = ::std::result::Result<T, Error>;

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
        static mut INITIALIZED: bool = false;
        unsafe {
            if INITIALIZED {
                return Err(From::from(Error::Initialized));
            } else {
                INITIALIZED = true;
            }
        }
    }
}
