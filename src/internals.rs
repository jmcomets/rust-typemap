use uany::{UnsafeAny, UnsafeAnyExt};
use std::any::Any;
use std::fmt::Debug;

/// Auto-implements `UnsafeAnyExt` for the given type, plus its Send/Sync combinations.
#[doc(hidden)] // Not actually exported
macro_rules! uany_ext {
    ( $x:tt ) => {
        unsafe impl UnsafeAnyExt for $x {}
        unsafe impl UnsafeAnyExt for $x + Send {}
        unsafe impl UnsafeAnyExt for $x + Sync {}
        unsafe impl UnsafeAnyExt for $x + Send + Sync {}
    }
}

///! Auto-implements `Implements` for the given type, plus its Send/Sync combinations.
#[doc(hidden)] // Not actually exported
macro_rules! implements {
    ( $x:tt ) => {
        unsafe impl<T: $x> Implements<$x> for T {
            fn into_object(self) -> Box<$x> {
                Box::new(self)
            }
        }

        unsafe impl<T: $x + Send> Implements<($x + Send)> for T {
            fn into_object(self) -> Box<$x + Send> {
                Box::new(self)
            }
        }

        unsafe impl<T: $x + Sync> Implements<($x + Sync)> for T {
            fn into_object(self) -> Box<$x + Sync> {
                Box::new(self)
            }
        }

        unsafe impl<T: $x + Send + Sync> Implements<($x + Send + Sync)> for T {
            fn into_object(self) -> Box<$x + Send + Sync> {
                Box::new(self)
            }
        }
    };
}

implements!(UnsafeAny);

/// A marker trait meant for use as the `A` parameter in `TypeMap`.
///
/// This can be used to construct `TypeMap`s containing only types which
/// implement `Debug` like so: `TypeMap::<DebugAny>::custom()`, which produces
/// a `TypeMap<DebugAny>`. Combine `DebugAny` with `Send` or `Sync` to add
/// additional bounds.
///
/// There is also an exported alias for this type of `TypeMap`, `DebugMap`.
pub trait DebugAny: Any + Debug { }
impl<T: Any + Debug> DebugAny for T { }

implements!(DebugAny);
uany_ext!(DebugAny);

/// A marker trait meant for use as the `A` parameter in `TypeMap`.
///
/// This can be used to construct `TypeMap`s containing only types which
/// implement `Clone` like so: `TypeMap::<CloneAny>::custom()`, which produces
/// a `TypeMap<CloneAny>`. Combine `CloneAny` with `Send` or `Sync` to add
/// additional bounds.
///
/// There is also an exported alias for this type of `TypeMap`, `CloneAny`.
pub trait CloneAny: Any {
    #[doc(hidden)]
    fn clone_any(&self) -> Box<CloneAny>;
    #[doc(hidden)]
    fn clone_any_send(&self) -> Box<CloneAny + Send> where Self: Send;
    #[doc(hidden)]
    fn clone_any_sync(&self) -> Box<CloneAny + Sync> where Self: Sync;
    #[doc(hidden)]
    fn clone_any_send_sync(&self) -> Box<CloneAny + Send + Sync> where Self: Send + Sync;
}

impl<T: Any + Clone> CloneAny for T {
    fn clone_any(&self) -> Box<CloneAny> { Box::new(self.clone()) }

    fn clone_any_send(&self) -> Box<CloneAny + Send> where Self: Send {
        Box::new(self.clone())
    }

    fn clone_any_sync(&self) -> Box<CloneAny + Sync> where Self: Sync {
        Box::new(self.clone())
    }

    fn clone_any_send_sync(&self) -> Box<CloneAny + Send + Sync>
    where Self: Send + Sync {
        Box::new(self.clone())
    }
}

implements!(CloneAny);
uany_ext!(CloneAny);

impl Clone for Box<CloneAny> {
    fn clone(&self) -> Box<CloneAny> { (**self).clone_any() }
}

impl Clone for Box<CloneAny + Send> {
    fn clone(&self) -> Box<CloneAny + Send> { (**self).clone_any_send() }
}

impl Clone for Box<CloneAny + Sync> {
    fn clone(&self) -> Box<CloneAny + Sync> { (**self).clone_any_sync() }
}

impl Clone for Box<CloneAny + Send + Sync> {
    fn clone(&self) -> Box<CloneAny + Send + Sync> { (**self).clone_any_send_sync() }
}

#[doc(hidden)] // Not actually exported
pub unsafe trait Implements<A: ?Sized + UnsafeAnyExt> {
    fn into_object(self) -> Box<A>;
}
