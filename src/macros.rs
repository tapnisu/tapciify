//! Just macros

#![allow(unused_imports)]

/// Macro for joining multiple iterators
#[macro_export]
macro_rules! product {
    ($first:ident, $($next:ident),*) => (
        $first.iter() $(
            .flat_map(|e| std::iter::repeat(e)
                .zip($next.iter()))
        )*
    );
}

pub(crate) use product;
