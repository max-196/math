macro_rules! implement_empty {
    ($trait_name:ident, $($type:ty),+) => {
        $(impl $trait_name for $type {})+
    };
}
pub(crate) use implement_empty;

macro_rules! implement_constant_function {
    ($trait_name:ident, $function_name:ident, $expression:expr, $($type:ty),+) => {
        $(impl $trait_name for $type {
            fn $function_name() -> Self {
                $expression
            }
        })+
    };
}
pub(crate) use implement_constant_function;