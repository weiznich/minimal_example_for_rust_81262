#![recursion_limit = "256"]
#[macro_use]
mod tuple;

pub trait SqlType {
    type IsNull: OneIsNullable<is_nullable::IsNullable> + OneIsNullable<is_nullable::NotNull>;
}

pub trait OneIsNullable<Other> {
    type Out: OneIsNullable<is_nullable::IsNullable> + OneIsNullable<is_nullable::NotNull>;
}

pub trait AllAreNullable<Other> {
    type Out: AllAreNullable<is_nullable::NotNull> + AllAreNullable<is_nullable::IsNullable>;
}

pub trait MaybeNullableType<O> {
    type Out: SqlType;
}

pub mod is_nullable {

    #[derive(Debug, Clone, Copy)]
    pub struct NotNull;

    #[derive(Debug, Clone, Copy)]
    pub struct IsNullable;
}

macro_rules! tuple_impls {
    ($(
        $Tuple:tt {
            $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)+
        }
    )+) => {
        $(
            impl_sql_type!($($T,)*);
        )*
    };
}

macro_rules! impl_sql_type {
    (
        @build
        start_ts = [$($ST: ident,)*],
        ts = [$T1: ident,],
        bounds = [$($bounds: tt)*],
        is_null = [$($is_null: tt)*],
    )=> {
        impl<$($ST,)*> SqlType for ($($ST,)*)
        where
            $($ST: SqlType,)*
            $($bounds)*
            $T1::IsNull: OneIsNullable<$($is_null)*>,
        {
            type IsNull = <$T1::IsNull as OneIsNullable<$($is_null)*>>::Out;
        }

    };
    (
        @build
        start_ts = [$($ST: ident,)*],
        ts = [$T1: ident, $($T: ident,)+],
        bounds = [$($bounds: tt)*],
        is_null = [$($is_null: tt)*],
    )=> {
        impl_sql_type!{
            @build
            start_ts = [$($ST,)*],
            ts = [$($T,)*],
            bounds = [$($bounds)* $T1::IsNull: OneIsNullable<$($is_null)*>,],
            is_null = [<$T1::IsNull as OneIsNullable<$($is_null)*>>::Out],
        }
    };
    ($T1: ident, $($T: ident,)+) => {
        impl_sql_type!{
            @build
            start_ts = [$T1, $($T,)*],
            ts = [$($T,)*],
            bounds = [],
            is_null = [$T1::IsNull],
        }
    };
    ($T1: ident,) => {
        impl<$T1> SqlType for ($T1,)
        where $T1: SqlType,
        {
            type IsNull = $T1::IsNull;
        }
    }
}

__diesel_for_each_tuple!(tuple_impls);

fn main() {
    println!("Hello, world!");
}
