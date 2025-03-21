use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

/// Any type whose any shared reference can be coerced to a `&dyn Any`.
pub trait AsAnyRef: Any {
    /// Returns a `&dyn Any` of `Self`.
    fn as_any_ref(&self) -> &dyn Any;

    /// Returns a `&(dyn Any + Send)` of `Self`.
    fn as_any_ref_send(&self) -> &(dyn Any + Send)
    where
        Self: Send;

    /// Returns a `&(dyn Any + Send + Sync)` of `Self`.
    fn as_any_ref_send_sync(&self) -> &(dyn Any + Send + Sync)
    where
        Self: Send + Sync;
}

impl<T: Any> AsAnyRef for T {
    #[inline]
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_ref_send(&self) -> &(dyn Any + Send)
    where
        Self: Send,
    {
        self
    }

    #[inline]
    fn as_any_ref_send_sync(&self) -> &(dyn Any + Send + Sync)
    where
        Self: Send + Sync,
    {
        self
    }
}

/// Any type whose any exclusive reference can be coerced to a `&mut dyn Any`.
pub trait AsAnyMut: AsAnyRef {
    /// Returns a `&mut dyn Any` of `Self`.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Returns a `&mut (dyn Any + Send)` of `Self`.
    fn as_any_mut_send(&mut self) -> &mut (dyn Any + Send)
    where
        Self: Send;

    /// Returns a `&mut (dyn Any + Send + Sync)` of `Self`.
    fn as_any_mut_send_sync(&mut self) -> &mut (dyn Any + Send + Sync)
    where
        Self: Send + Sync;
}

impl<T: AsAnyRef> AsAnyMut for T {
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline]
    fn as_any_mut_send(&mut self) -> &mut (dyn Any + Send)
    where
        Self: Send,
    {
        self
    }

    #[inline]
    fn as_any_mut_send_sync(&mut self) -> &mut (dyn Any + Send + Sync)
    where
        Self: Send + Sync,
    {
        self
    }
}

/// Any type whose any boxed representation can be coerced to a `Box<dyn Any>`.
pub trait IntoAnyBox: AsAnyMut {
    /// Consumes the value and returns a `Box<dyn Any>`.
    fn into_any_box(self: Box<Self>) -> Box<dyn Any>;

    /// Consumes the value and returns a `Box<dyn Any + Send>`.
    fn into_any_box_send(self: Box<Self>) -> Box<dyn Any + Send>
    where
        Self: Send;

    /// Consumes the value and returns a `Box<dyn Any + Send + Sync>`.
    fn into_any_box_send_sync(self: Box<Self>) -> Box<dyn Any + Send + Sync>
    where
        Self: Send + Sync;
}

impl<T: AsAnyMut> IntoAnyBox for T {
    #[inline]
    fn into_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    #[inline]
    fn into_any_box_send(self: Box<Self>) -> Box<dyn Any + Send>
    where
        Self: Send,
    {
        self
    }

    #[inline]
    fn into_any_box_send_sync(self: Box<Self>) -> Box<dyn Any + Send + Sync>
    where
        Self: Send + Sync,
    {
        self
    }
}

/// Any type whose any `Rc` boxed representation can be coerced to a `Rc<dyn Any>`.
pub trait IntoAnyRc: AsAnyRef {
    /// Consumes the value and returns a `Rc<dyn Any>`.
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Any>;

    /// Consumes the value and returns a `Rc<dyn Any + Send>`.
    fn into_any_rc_send(self: Rc<Self>) -> Rc<dyn Any + Send>
    where
        Self: Send;

    /// Consumes the value and returns a `Rc<dyn Any + Send + Sync>`.
    fn into_any_rc_send_sync(self: Rc<Self>) -> Rc<dyn Any + Send + Sync>
    where
        Self: Send + Sync;
}

impl<T: AsAnyRef> IntoAnyRc for T {
    #[inline]
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Any> {
        self
    }

    #[inline]
    fn into_any_rc_send(self: Rc<Self>) -> Rc<dyn Any + Send>
    where
        Self: Send,
    {
        self
    }

    #[inline]
    fn into_any_rc_send_sync(self: Rc<Self>) -> Rc<dyn Any + Send + Sync>
    where
        Self: Send + Sync,
    {
        self
    }
}

/// Any type whose any `Arc` boxed representation can be coerced to a `Arc<dyn Any>`.
pub trait IntoAnyArc: AsAnyRef {
    /// Consumes the value and returns a `Arc<dyn Any>`.
    fn into_any_arc(self: Arc<Self>) -> Arc<dyn Any>;

    /// Consumes the value and returns a `Arc<dyn Any + Send>`.
    fn into_any_arc_send(self: Arc<Self>) -> Arc<dyn Any + Send>
    where
        Self: Send;

    /// Consumes the value and returns a `Arc<dyn Any + Send + Sync>`.
    fn into_any_arc_send_sync(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>
    where
        Self: Send + Sync;
}

impl<T: AsAnyRef> IntoAnyArc for T {
    #[inline]
    fn into_any_arc(self: Arc<Self>) -> Arc<dyn Any> {
        self
    }

    #[inline]
    fn into_any_arc_send(self: Arc<Self>) -> Arc<dyn Any + Send>
    where
        Self: Send,
    {
        self
    }

    #[inline]
    fn into_any_arc_send_sync(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>
    where
        Self: Send + Sync,
    {
        self
    }
}
