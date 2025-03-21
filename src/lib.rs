//! ## Overview
//!
//! Better As-any is a refined implementation of the
//! [`as-any`](https://crates.io/crates/as-any) crate. This crate avoids its
//! predecessor's caveats that the API is error-prone when smart pointers are
//! involved and it's impossible to downcasting an owned smart pointer.
//!
//! With `as-any` crate, you can't directly call `is` or `downcast*` methods on
//! a smart pointer, since it simply takes the smart pointer's reference and
//! leads to a runtime error, which is hardly to be figure out.
//!
//! ## Usage
//!
//! Make your traits inherit the [`InheritAny`] trait, and all necessary
//! functionalities will be added to your traits' implementors, including trait
//! objects.
//!
//! When downcasting is needed, corresponding helper traits including
//! [`DowncastRef`], [`DowncastMut`] and [`Downcast`] are expected to be
//! imported.
//!
//! ```rust
//! use std::fmt::Debug;
//! use std::sync::Arc;
//!
//! use better_as_any::{InheritAny, DowncastRef, Downcast};
//!
//! pub trait Trait: InheritAny + Debug + Send + Sync {}
//!
//! impl Trait for i32 {}
//!
//! let val: Box<dyn Trait> = Box::new(42i32);
//! assert!(val.is::<i32>()); // No need to use `(*val).is::<i32>()`.
//!
//! let val: Arc<dyn Trait> = Arc::from(val);
//! assert_eq!(*val.downcast_ref::<i32>().unwrap(), 42i32);
//! assert_eq!(val.downcast::<i32>().unwrap(), Arc::new(42i32)); // Downcasts the `Arc`.
//! ```

pub mod downcasting;
pub mod upcasting;

use std::any::{self, Any};

pub use downcasting::{Downcast, DowncastMut, DowncastRef};
use upcasting::{AsAnyMut, AsAnyRef, IntoAnyArc, IntoAnyBox, IntoAnyRc};

/// Trait which adds runtime reflection functionality to all its implementors
pub trait InheritAny: Any + AsAnyRef + AsAnyMut + IntoAnyBox + IntoAnyRc + IntoAnyArc {
    /// Returns the concrete type's name.
    ///
    /// Note that developers should not rely on the content of the value, since
    /// it's subjected to the internal implementation of rustc, and may varies
    /// on different machines, compilers or builds. See
    /// [`std::any::type_name()`] for more information.
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
