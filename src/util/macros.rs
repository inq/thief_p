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

        impl ::ui::comp::View for $target {
            def_child!(@child_mut $target, [$($src),*], get_view_mut(): &mut ::ui::comp::ViewT);
            def_child!(@child_imm $target, [$($src),*], get_view(): &::ui::comp::ViewT);
        }

        impl ::ui::comp::Component for $target {
            def_child!(@child_mut $target, [$($src),*], on_resize(workspace: &mut ::hq::Workspace): ResultBox<()>);
            def_child!(@child_mut $target, [$($src),*], refresh(workspace: &mut ::hq::Workspace): ResultBox<::ui::Response>);
            def_child!(@child_mut $target, [$($src),*],
                       unhandled(workspace: &mut ::hq::Workspace, e: ::ui::Request): ResultBox<::ui::Response>);
            def_child!(@child_mut $target, [$($src),*],
                       handle(workspace: &mut ::hq::Workspace, e: ::ui::Request): ResultBox<::ui::Response>);
            def_child!(@child_mut $target, [$($src),*],
                       on_key(workspace: &mut ::hq::Workspace, k: ::term::Key): ResultBox<::ui::Response>);
        }
    };
}

macro_rules! def_error {
    ( $($x:ident: $y:expr,)* ) => {
        #[derive(Debug)]
        pub enum Error {
            $($x,)*
        }

        #[allow(dead_code)]
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
