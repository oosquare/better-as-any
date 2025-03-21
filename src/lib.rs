pub mod downcasting;
pub mod upcasting;

use std::any::{self, Any};

pub use upcasting::{AsAnyMut, AsAnyRef, IntoAnyArc, IntoAnyBox, IntoAnyRc};

pub trait InheritAny: Any + AsAnyRef + AsAnyMut + IntoAnyBox + IntoAnyRc + IntoAnyArc {
    fn type_name(&self) -> &'static str;
}

impl<T: Any> InheritAny for T {
    fn type_name(&self) -> &'static str {
        any::type_name::<T>()
    }
}

#[cfg(test)]
mod tests {
    use any::TypeId;

    use super::*;

    trait Trait: InheritAny {}

    impl Trait for i32 {}

    #[test]
    fn dyn_trait_type_id_succeeds() {
        let val = 42i32;
        let val_ref: &dyn Trait = &val;
        assert_eq!(val_ref.type_id(), TypeId::of::<i32>());
    }

    #[test]
    fn dyn_trait_type_name_succeeds() {
        let val = 42i32;
        let val_ref: &dyn Trait = &val;
        assert_eq!(val_ref.type_name(), any::type_name_of_val(&val));
    }
}
