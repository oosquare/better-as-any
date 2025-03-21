pub mod applicable;
pub mod owned;

use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use applicable::Applicable;
use owned::{Owned, OwnedFamily};

use crate::{AsAnyMut, AsAnyRef, IntoAnyArc, IntoAnyBox, IntoAnyRc};

/// Helper trait which adds shared reference downcasting functionality
/// automatically for any type implementing [`AsAnyRef`].
pub trait DowncastRef {
    /// Returns true if the underlying type is `T`.
    fn is<T: Any>(&self) -> bool;

    /// Tries to downcast to `&T`.
    fn downcast_ref<T: Any>(&self) -> Option<&T>;
}

impl<S: Deref<Target: AsAnyRef>> DowncastRef for S {
    #[inline]
    fn is<T: Any>(&self) -> bool {
        (**self).as_any_ref().is::<T>()
    }

    #[inline]
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (**self).as_any_ref().downcast_ref::<T>()
    }
}

/// Helper trait which adds exclusive reference downcasting functionality
/// automatically for any type implementing [`AsAnyMut`].
pub trait DowncastMut {
    /// Tries to downcast to `&mut T`.
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T>;
}

impl<S: DerefMut<Target: AsAnyMut>> DowncastMut for S {
    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        (**self).as_any_mut().downcast_mut::<T>()
    }
}

/// Helper trait which adds smart pointer downcasting functionality
/// automatically for any capable type.
pub trait Downcast: Owned + Sized {
    /// Tries to downcast to concrete boxed type.
    ///
    /// Technically, whether a dynamic type can be downcast to a concrete type
    /// is determined by both `T` and the smart pointer (i.e. the implementor
    /// of `Downcast`). The [`Applicable`] models this double dispatch
    /// mechanism.
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>;
}

impl<S: IntoAnyBox> Downcast for Box<S> {
    /// Tries to downcast to concrete boxed type.
    ///
    /// Any `T` implementing `Any` is effectively acceptable for this method.
    /// See implementors of [`Applicable`] for more informations.
    #[inline]
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>,
    {
        T::apply_downcasting(self)
    }
}

impl<S: IntoAnyRc> Downcast for Rc<S> {
    /// Tries to downcast to concrete boxed type.
    ///
    /// Any `T` implementing `Any` is effectively acceptable for this method.
    /// See implementors of [`Applicable`] for more informations.
    #[inline]
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>,
    {
        T::apply_downcasting(self)
    }
}

impl<S: IntoAnyArc> Downcast for Arc<S> {
    /// Tries to downcast to concrete boxed type.
    ///
    /// Any `T` implementing `Any + Send + Sync` is effectively acceptable for
    /// this method. See implementors of [`Applicable`] for more informations.
    #[inline]
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>,
    {
        T::apply_downcasting(self)
    }
}
