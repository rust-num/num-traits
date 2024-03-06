// not all are used in all features configurations
#![allow(unused)]

/// Forward a method to an inherent method or a base trait method.
macro_rules! forward {
    ($( Self :: $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(self $( , $arg : $ty )* ) -> $ret {
                Self::$method(self $( , $arg )* )
            }
        )*};
    ($( $base:ident :: $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(self $( , $arg : $ty )* ) -> $ret {
                <Self as $base>::$method(self $( , $arg )* )
            }
        )*};
    ($( $base:ident :: $method:ident ( $( $arg:ident : $ty:ty ),* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method( $( $arg : $ty ),* ) -> $ret {
                <Self as $base>::$method( $( $arg ),* )
            }
        )*};
    ($( $imp:path as $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
        => {$(
            #[inline]
            fn $method(self $( , $arg : $ty )* ) -> $ret {
                $imp(self $( , $arg )* )
            }
        )*};
    ($( $base:ident :: $associate:ident ; )*)
        => {$(
            const $associate: Self = $base::$associate;
        )*};
}

macro_rules! constant {
    ($( $associate:ident = $ret:expr ; )*)
        => {$(
            const $associate: Self = $ret;
        )*};

        ($( $associate:ident $type:ty = $ret:expr ; )*)
        => {$(
            const $associate: &type = $ret;
        )*};
}