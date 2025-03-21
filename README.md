# Better As-Any

[GitHub](https://github.com/oosquare/better-as-any) | [Crates.io](https://crates.io/crates/better-as-any) | [Docs.rs](https://docs.rs/better-as-any)

## Overview

Better As-any is a refined implementation of the [`as-any`](https://crates.io/crates/as-any) crate. This crate avoids its predecessor's caveats that the API is error-prone when smart pointers are involved and it's impossible to downcasting an owned smart pointer.

With `as-any` crate, you can't directly call `is` or `downcast*` methods on a smart pointer, since it simply takes the smart pointer's reference and leads to a runtime error, which is hardly to be figure out.

## Usage

Make your traits inherit the `InheritAny` trait, and all necessary functionalities will be added to your traits' implementors, including trait objects.

When downcasting is needed, corresponding helper traits including `DowncastRef`, `DowncastMut` and `Downcast` are expected to be imported.

```rust
use std::fmt::Debug;
use std::sync::Arc;

use better_as_any::{InheritAny, DowncastRef, Downcast};

pub trait Trait: InheritAny + Debug + Send + Sync {}

impl Trait for i32 {}

let val: Box<dyn Trait> = Box::new(42i32);
assert!(val.is::<i32>()); // No need to use `(*val).is::<i32>()`.

let val: Arc<dyn Trait> = Arc::from(val);
assert_eq!(*val.downcast_ref::<i32>().unwrap(), 42i32);
assert_eq!(val.downcast::<i32>().unwrap(), Arc::new(42i32)); // Downcasts the `Arc`.
```
