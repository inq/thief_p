/// Boilerplate for ui elements which have a view.
macro_rules! has_view {
    () => {
        fn get_view_mut(&mut self) -> &mut View { &mut self.view }
        fn get_view(&self) -> &View { &self.view }
    }
}

/// Merge elements into a enum. Default value is the first element.
macro_rules! def_child {
    ( @default $target:ident, $first:ident) => {
        // Default value implementation
        impl Default for $target {
            fn default() -> $target {
                $target::$first(Default::default())
            }
        }
    };
    ( @default $target:ident, $first:ident, $($r:ident),* ) => {
        // Default value implementation
        impl Default for $target {
            fn default() -> $target {
                $target::$first(Default::default())
            }
        }
    };
    ( @inner $child: ident, $func:ident, [$($arg:ident),*] ) => {
        // The innermost part
        $child.$func($($arg,)*)
    };
    ( @match_mut $target:ident, $s:ident, $func:ident, [$($src:ident),*], $args:tt ) => {
        // The match body of the mutable form
        match *$s {
            $( $target::$src(ref mut _child) => def_child!(@inner _child, $func, $args) ),*
        }
    };
    ( @match_imm $target:ident, $s:ident, $func:ident, [$($src:ident),*], $args:tt ) => {
        // The match body of the immutable form
        match *$s {
            $( $target::$src(ref _child) => def_child!(@inner _child, $func, $args) ),*
        }
    };
    ( @child_mut $target:ident, $srcs:tt, $x:ident( $($arg:ident: $argt:ty),* ): $y:ty ) => {
        // Mutable form of Component implementation
        fn $x(&mut self, $( $arg: $argt,)*) -> $y {
            def_child!(@match_mut $target, self, $x, $srcs, [$($arg),*] )
        }
    };
    ( @child_imm $target:ident, $srcs:tt, $x:ident( $($arg:ident: $argt:ty),* ): $y:ty ) => {
        // Immutable form of Component implementation
        fn $x(&self, $( $arg: $argt,)*) -> $y {
            def_child!(@match_imm $target, self, $x, $srcs, [$($arg),*] )
        }
    };
    ( $target:ident <- $($src:ident),* ) => {
        // The entry point
        pub enum $target {
            $($src($src)),*
        }
        def_child!(@default $target, $($src),*);
        impl Component for $target {
            def_child!(@child_mut $target, [$($src),*], get_view_mut(): &mut View);
            def_child!(@child_imm $target, [$($src),*], get_view(): &View);
            def_child!(@child_mut $target, [$($src),*], on_resize(): ());
            def_child!(@child_imm $target, [$($src),*], refresh(hq: &mut Hq): ResultBox<Response>);
            def_child!(@child_mut $target, [$($src),*],
                       handle(e: Event, hq: &mut Hq): ResultBox<Response>);
        }
    };
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
