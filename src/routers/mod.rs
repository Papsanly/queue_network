mod direct;
mod probability;

use crate::blocks::BlockId;
pub use direct::DirectRouter;
pub use probability::ProbabilityRouter;

pub trait Router {
    fn next(&self) -> Option<BlockId>;
}

macro_rules! impl_router {
    ($enum_name:ident {$($name:ident),*}) => {
        pub enum $enum_name {
            $($name($name),)*
        }

        $(
            impl From<$name> for $enum_name {
                fn from(router: $name) -> Self {
                    RouterType::$name(router)
                }
            }
        )*

        impl Router for $enum_name {
            fn next(&self) -> Option<BlockId> {
                match self {
                    $($enum_name::$name(router) => router.next(),)*
                }
            }
        }
    };
}

impl_router!(RouterType {
    DirectRouter,
    ProbabilityRouter
});
