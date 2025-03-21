pub mod applicable;
pub mod owned;

use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use applicable::Applicable;
use owned::{Owned, OwnedFamily};

use crate::{AsAnyMut, AsAnyRef, IntoAnyArc, IntoAnyBox, IntoAnyRc};

pub trait DowncastRef {
    fn is<T: Any>(&self) -> bool;

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

pub trait DowncastMut {
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T>;
}

impl<S: DerefMut<Target: AsAnyMut>> DowncastMut for S {
    #[inline]
    fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        (**self).as_any_mut().downcast_mut::<T>()
    }
}

pub trait Downcast: Owned + Sized {
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>;
}

impl<S: IntoAnyBox> Downcast for Box<S> {
    #[inline]
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>,
    {
        T::apply_downcasting(self)
    }
}

impl<S: IntoAnyRc> Downcast for Rc<S> {
    #[inline]
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>,
    {
        T::apply_downcasting(self)
    }
}

impl<S: IntoAnyArc> Downcast for Arc<S> {
    #[inline]
    fn downcast<T>(self) -> Result<T::Output, Self>
    where
        T: Applicable<Self, Output = <Self::Family as OwnedFamily>::Owned<T>>,
    {
        T::apply_downcasting(self)
    }
}
