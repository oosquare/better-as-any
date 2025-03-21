use std::{any::Any, rc::Rc, sync::Arc};

pub trait AsAnyRef: Any {
    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_ref_send(&self) -> &(dyn Any + Send)
    where
        Self: Send;

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

pub trait AsAnyMut: AsAnyRef {
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any_mut_send(&mut self) -> &mut (dyn Any + Send)
    where
        Self: Send;

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

pub trait IntoAnyBox: AsAnyMut {
    fn into_any_box(self: Box<Self>) -> Box<dyn Any>;

    fn into_any_box_send(self: Box<Self>) -> Box<dyn Any + Send>
    where
        Self: Send;

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

pub trait IntoAnyRc: AsAnyRef {
    fn into_any_rc(self: Rc<Self>) -> Rc<dyn Any>;

    fn into_any_rc_send(self: Rc<Self>) -> Rc<dyn Any + Send>
    where
        Self: Send;

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

pub trait IntoAnyArc: AsAnyRef {
    fn into_any_arc(self: Arc<Self>) -> Arc<dyn Any>;

    fn into_any_arc_send(self: Arc<Self>) -> Arc<dyn Any + Send>
    where
        Self: Send;

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
