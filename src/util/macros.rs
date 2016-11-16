/// Boilerplate for ui elements which have a view.

macro_rules! has_view {
    () => {
        fn get_view_mut(&mut self) -> &mut View { &mut self.view }
        fn get_view(&self) -> &View { &self.view }
    }
}

macro_rules! def_child {
    ( $target:ident <- $($src:ident),* ) => {
        pub enum $target {
            $($src($src)),*
        }

        impl Default for ScreenChild {
            fn default() -> ScreenChild {
                ScreenChild::HSplit(Default::default())
            }
        }

        impl Component for ScreenChild {
            def_child_func!(mut get_view_mut(): &mut View);
            def_child_func!(get_view(): &View);
            def_child_func!(mut on_resize(): ());
            def_child_func!(refresh(): Response);
            def_child_func!(mut handle(e: Event): Response);
        }
    }
}

macro_rules! def_child_func {
    ( @inner $h: ident, $x:ident, [$($arg:ident),*] ) => {
        $h.$x($($arg,)*)
    };
    ( @outer $s:ident, $x:ident, [$($vs:ident),*], $rem:tt ) => {
        match *$s {
            $( ScreenChild::$vs(ref sc) => def_child_func!(@inner sc, $x, $rem) ),*
        }
    };
    ( @outer_mut $s:ident, $x:ident, [$($vs:ident),*], $rem:tt ) => {
        match *$s {
            $( ScreenChild::$vs(ref mut sc) => def_child_func!(@inner sc, $x, $rem) ),*
        }
    };
    ( mut $x:ident ( $($arg:ident: $argt:ty),* ): $y:ty ) => {
        fn $x(&mut self, $( $arg: $argt,)*) -> $y {
            def_child_func!(@outer_mut self, $x, [CommandBar, HSplit], [$($arg),*] )
        }
    };
    ( $x:ident ( $($arg:ident: $argt:ty),* ): $y:ty ) => {
        fn $x(&self, $($arg,)*) -> $y {
            def_child_func!(@outer self, $x, [CommandBar, HSplit], [$($arg),*] )
        }
    }
}

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
