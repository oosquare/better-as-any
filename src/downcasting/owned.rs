use std::rc::Rc;
use std::sync::Arc;

pub trait Owned {
    type Family: OwnedFamily;
}

impl<T: ?Sized> Owned for Box<T> {
    type Family = BoxFamily;
}

impl<T: ?Sized> Owned for Rc<T> {
    type Family = RcFamily;
}

impl<T: ?Sized> Owned for Arc<T> {
    type Family = ArcFamily;
}

pub trait OwnedFamily {
    type Owned<T: ?Sized>: Owned<Family = Self>;
}

pub struct BoxFamily;

impl OwnedFamily for BoxFamily {
    type Owned<T: ?Sized> = Box<T>;
}

pub struct RcFamily;

impl OwnedFamily for RcFamily {
    type Owned<T: ?Sized> = Rc<T>;
}

pub struct ArcFamily;

impl OwnedFamily for ArcFamily {
    type Owned<T: ?Sized> = Arc<T>;
}
