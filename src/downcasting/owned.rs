use std::rc::Rc;
use std::sync::Arc;

/// A owned wrapper along with the content it holds.
///
/// For example, `Box<i32>`, `Rc<f64>` and so on implement this.
pub trait Owned {
    /// The family of the owned wrapper, used as a type function which returns
    /// this kind of wrapper containing any type.
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

/// The family of a series of similar owned wrapper.
///
/// Since Rust doesn't have directly Higher Kinded Type (HKT) support,
/// `Box<_>`, `Rc<_>` and so on are represented using Generic Associated Type
/// (GAT) in these families.
pub trait OwnedFamily {
    /// A type function which returns a owned wrapper of `T` in this family.
    type Owned<T: ?Sized>: Owned<Family = Self>;
}

/// The family of all `Box<T>`.
pub struct BoxFamily;

impl OwnedFamily for BoxFamily {
    type Owned<T: ?Sized> = Box<T>;
}

/// The family of all `Rc<T>`.
pub struct RcFamily;

impl OwnedFamily for RcFamily {
    type Owned<T: ?Sized> = Rc<T>;
}

/// The family of all `Arc<T>`.
pub struct ArcFamily;

impl OwnedFamily for ArcFamily {
    type Owned<T: ?Sized> = Arc<T>;
}
