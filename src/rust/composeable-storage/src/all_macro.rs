#[macro_export]
macro_rules! all {
    ( #[cfg($meta:meta)] $($item:item)* ) => {
        $(#[cfg($meta)] $item)*
    };
}