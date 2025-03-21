use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

use crate::downcasting::owned::{Owned, OwnedFamily};
use crate::downcasting::DowncastRef;
use crate::upcasting::{IntoAnyArc, IntoAnyBox, IntoAnyRc};

/// Trait that models double dispatch in type downcasting.
///
/// This trait is implemented by all types used as target type in downcasting,
/// while `O` is for a owned wrapper of a trait object, usually a smart pointer.
pub trait Applicable<O>: Any + Sized
where
    O: Owned,
    O::Family: OwnedFamily<Owned<Self> = Self::Output>,
{
    /// The type of a boxed `Self`.
    type Output;

    /// Downcasts the owned wrapper to a wrapper of a concrete type.
    fn apply_downcasting(owned: O) -> Result<Self::Output, O>;
}

impl<S, T> Applicable<Box<S>> for T
where
    S: IntoAnyBox + ?Sized,
    T: Any,
{
    type Output = Box<T>;

    fn apply_downcasting(owned: Box<S>) -> Result<Self::Output, Box<S>> {
        if owned.is::<T>() {
            let res = owned
                .into_any_box()
                .downcast::<T>()
                .unwrap_or_else(|_| std::unreachable!("`self` should be `Box<T>`"));
            Ok(res)
        } else {
            Err(owned)
        }
    }
}

impl<S, T> Applicable<Rc<S>> for T
where
    S: IntoAnyRc + ?Sized,
    T: Any,
{
    type Output = Rc<T>;

    fn apply_downcasting(owned: Rc<S>) -> Result<Self::Output, Rc<S>> {
        if owned.is::<T>() {
            let res = owned
                .into_any_rc()
                .downcast::<T>()
                .unwrap_or_else(|_| std::unreachable!("`self` should be `Rc<T>`"));
            Ok(res)
        } else {
            Err(owned)
        }
    }
}

impl<S, T> Applicable<Arc<S>> for T
where
    S: IntoAnyArc + Send + Sync + ?Sized,
    T: Any + Send + Sync,
{
    type Output = Arc<T>;

    fn apply_downcasting(owned: Arc<S>) -> Result<Self::Output, Arc<S>> {
        if owned.is::<T>() {
            let res = owned
                .into_any_arc_send_sync()
                .downcast::<T>()
                .unwrap_or_else(|_| std::unreachable!("`self` should be `Arc<T>`"));
            Ok(res)
        } else {
            Err(owned)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait Trait: IntoAnyBox + IntoAnyRc + IntoAnyArc + Send + Sync {}

    impl Trait for i32 {}

    #[test]
    fn applicable_box() {
        let val: Box<dyn Trait> = Box::new(42i32);

        let val: Box<i32> = i32::apply_downcasting(val).unwrap_or_else(|_| unreachable!());
        assert_eq!(*val, 42i32);

        let val: Box<i32> = i32::apply_downcasting(val).unwrap();
        assert_eq!(*val, 42i32);
    }

    #[test]
    fn applicable_rc() {
        let owned = Rc::new(42i32);
        let val = Rc::clone(&owned);
        let val: Rc<dyn Trait> = val;

        let val: Rc<i32> = i32::apply_downcasting(val).unwrap_or_else(|_| unreachable!());
        assert_eq!(*val, 42i32);

        let val: Rc<i32> = i32::apply_downcasting(val).unwrap();
        assert_eq!(*val, 42i32);
    }

    #[test]
    fn applicable_arc() {
        let owned = Arc::new(42i32);
        let val = Arc::clone(&owned);
        let val: Arc<dyn Trait> = val;

        let val: Arc<i32> = i32::apply_downcasting(val).unwrap_or_else(|_| unreachable!());
        assert_eq!(*val, 42i32);

        let val: Arc<i32> = i32::apply_downcasting(val).unwrap();
        assert_eq!(*val, 42i32);
    }
}
