/// Boilerplate for ui elements which have a view.
#[macro_export]
macro_rules! has_view {
    () => {
        fn get_view_mut(&mut self) -> &mut View { &mut self.view }
        fn get_view(&self) -> &View { &self.view }
    }
}

#[macro_export]
macro_rules! def_child_func {
    ( mut $x:ident: $y:ty ) => {
        fn $x(&mut self) -> $y {
            match *self {
                ScreenChild::CommandBar(ref mut sc) => sc.$x(),
                ScreenChild::HSplit(ref mut sc) => sc.$x(),
            }
        }
    };
    ( $x:ident: $y:ty ) => {
        fn $x(&self) -> $y {
            match *self {
                ScreenChild::CommandBar(ref sc) => sc.$x(),
                ScreenChild::HSplit(ref sc) => sc.$x(),
            }
        }
    }
}

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
