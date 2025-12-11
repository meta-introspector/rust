//! This module defines custom marker traits `DynSend` and `DynSync` and associated utilities.
//! These traits are used to control thread-safety properties dynamically at runtime,
//! particularly in scenarios where the standard `Send` and `Sync` traits are too restrictive
//! or need to be overridden based on a runtime flag (`sync::is_dyn_thread_safe`).
//!
//! The module provides macros to implement these auto traits both positively and negatively
//! for various standard library types and custom data structures, ensuring consistency
//! with Rust's fundamental thread-safety guarantees while allowing for dynamic behavior.

use std::alloc::Allocator;
use std::marker::PointeeSized;

#[diagnostic::on_unimplemented(message = "`{Self}` doesn't implement `DynSend`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Send`")]
// This is an auto trait for types which can be sent across threads if `sync::is_dyn_thread_safe()`
// is true. These types can be wrapped in a `FromDyn` to get a `Send` type. Wrapping a
// `Send` type in `IntoDynSyncSend` will create a `DynSend` type.
pub unsafe auto trait DynSend {}

#[diagnostic::on_unimplemented(message = "`{Self}` doesn't implement `DynSync`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Sync`")]
// This is an auto trait for types which can be shared across threads if `sync::is_dyn_thread_safe()`
// is true. These types can be wrapped in a `FromDyn` to get a `Sync` type. Wrapping a
// `Sync` type in `IntoDynSyncSend` will create a `DynSync` type.
pub unsafe auto trait DynSync {}

// The `DynSend` trait is automatically implemented for references (`&T`)
// if the referenced type `T` is `DynSync`. This ensures that if a type
// can be safely shared across threads (`DynSync`), then a reference to it
// can also be sent to another thread (`DynSend`). This is consistent with
// how `Send` and `Sync` interact in Rust's standard library.
unsafe impl<T: DynSync + ?Sized + PointeeSized> DynSend for &T {}

macro_rules! impls_dyn_send_neg {
    ($([$t1: ty $(where $($generics1: tt)*)?])*) => {
        $(impl$(<$($generics1)*>)? !DynSend for $t1 {})*
    };
}

// Negatively implement `DynSend` for types that are typically not `Send`
// in the standard library. This ensures that `DynSend` mirrors the behavior
// of `Send` for these specific types, preventing them from being
// dynamically sent across threads.
impls_dyn_send_neg!(
    [std::env::Args]
    [std::env::ArgsOs]
    [*const T where T: ?Sized + PointeeSized]
//    [*mut T where T: ?Sized + PointeeSized]
//    [std::ptr::NonNull<T> where T: ?Sized + PointeeSized]
    [std::rc::Rc<T, A> where T: ?Sized, A: Allocator]
    [std::rc::Weak<T, A> where T: ?Sized, A: Allocator]
    [std::sync::MutexGuard<'_, T> where T: ?Sized]
    [std::sync::RwLockReadGuard<'_, T> where T: ?Sized]
    [std::sync::RwLockWriteGuard<'_, T> where T: ?Sized]
    [std::io::StdoutLock<'_>]
    [std::io::StderrLock<'_>]
);

#[cfg(any(
    unix,
    target_os = "hermit",
    all(target_vendor = "fortanix", target_env = "sgx"),
    target_os = "solid_asp3",
    target_os = "wasi",
    target_os = "xous"
))]
// Consistent with `std`, `env_imp::Env` is `!Sync` in these platforms
impl !DynSend for std::env::VarsOs {}

macro_rules! already_send {
    ($([$ty: ty])*) => {
        $(unsafe impl DynSend for $ty where Self: Send {})*
    };
}

// Implements `DynSend` for types that are already `Send`.
// This is a convenient way to bring existing `Send` types into the `DynSend` ecosystem.
already_send!(
    [std::backtrace::Backtrace][std::io::Stdout][std::io::Stderr][std::io::Error][std::fs::File][std::panic::Location<'_>]
        [rustc_arena::DroplessArena][jobserver_crate::Client][jobserver_crate::HelperThread]
        [crate::memmap::Mmap][crate::profiling::SelfProfiler][crate::owned_slice::OwnedSlice]
);

macro_rules! impl_dyn_send {
    ($($($attr: meta)* [$ty: ty where $($generics2: tt)*])*) => {
        $(unsafe impl<$($generics2)*> DynSend for $ty {})*
    };
}

// Explicitly implements `DynSend` for types that are known to be safe to send
// dynamically, often with specific generic bounds. This macro allows for
// fine-grained control over `DynSend` implementations for complex types.
impl_dyn_send!(
    [std::sync::atomic::AtomicPtr<T> where T]
    [std::sync::Mutex<T> where T: ?Sized+ DynSend]
    [std::sync::mpsc::Sender<T> where T: DynSend]
    [std::sync::Arc<T> where T: ?Sized + DynSync + DynSend]
    [std::sync::LazyLock<T, F> where T: DynSend, F: DynSend]
    [std::collections::HashSet<K, S> where K: DynSend, S: DynSend]
    [std::collections::HashMap<K, V, S> where K: DynSend, V: DynSend, S: DynSend]
    [std::collections::BTreeMap<K, V, A> where K: DynSend, V: DynSend, A: std::alloc::Allocator + Clone + DynSend]
    [Vec<T, A> where T: DynSend, A: std::alloc::Allocator + DynSend]
    [Box<T, A> where T: ?Sized + DynSend, A: std::alloc::Allocator + DynSend]
    [crate::sync::RwLock<T> where T: DynSend]
    [crate::tagged_ptr::TaggedRef<'a, P, T> where 'a, P: Sync, T: Send + crate::tagged_ptr::Tag]
    [rustc_arena::TypedArena<T> where T: DynSend]
    [hashbrown::HashTable<T> where T: DynSend]
    [indexmap::IndexSet<V, S> where V: DynSend, S: DynSend]
    [indexmap::IndexMap<K, V, S> where K: DynSend, V: DynSend, S: DynSend]
    [thin_vec::ThinVec<T> where T: DynSend]
    // // [smallvec::SmallVec<A> where A: smallvec::Array + DynSend]
);

// Blanket `DynSend` implementation for `NonNull<T>`.
// This is marked `unsafe` because `DynSend` is an `unsafe auto trait`.
// The safety here relies on `T` itself being `Send`, which ensures that
// the raw pointer wrapped by `NonNull` can be safely sent across threads.
unsafe impl<T: ?Sized + PointeeSized + Send> DynSend for std::ptr::NonNull<T> {}

// Blanket `DynSync` implementation for `NonNull<T>`.
// This is marked `unsafe` because `DynSync` is an `unsafe auto trait`.
// The safety here relies on `T` itself being `Sync`, which ensures that
// the raw pointer wrapped by `NonNull` can be safely shared across threads.
//
// NOTE: A previous negative implementation for `NonNull<T>` existed within
// `impls_dyn_sync_neg!`, which led to a conflicting implementation error (E0751).
// That negative implementation has been removed to resolve the conflict,
// as this positive blanket `impl` is the intended and correct behavior when `T: Sync`.
unsafe impl<T: ?Sized + PointeeSized + Sync> DynSync for std::ptr::NonNull<T> {}

macro_rules! impls_dyn_sync_neg {
    ($([$t1: ty $(where $($generics1: tt)*)?])*) => {
        $(impl$(<$($generics1)*>)? !DynSync for $t1 {})*
    };
}

// Negatively implement `DynSync` for types that are typically not `Sync`
// in the standard library. This ensures that `DynSync` mirrors the behavior
// of `Sync` for these specific types, preventing them from being
// dynamically shared across threads.
impls_dyn_sync_neg!(
    [std::env::Args]
    [std::env::ArgsOs]
    [*const T where T: ?Sized + PointeeSized]
    [*mut T where T: ?Sized + PointeeSized]
    [std::cell::Cell<T> where T: ?Sized]
    [std::cell::RefCell<T> where T: ?Sized]
    [std::cell::UnsafeCell<T> where T: ?Sized]
    [std::rc::Rc<T, A> where T: ?Sized, A: Allocator]
    [std::rc::Weak<T, A> where T: ?Sized, A: Allocator]
    [std::cell::OnceCell<T> where T]
    [std::sync::mpsc::Receiver<T> where T]
    [std::sync::mpsc::Sender<T> where T]
);

#[cfg(any(
    unix,
    target_os = "hermit",
    all(target_vendor = "fortanix", target_env = "sgx"),
    target_os = "solid_asp3",
    target_os = "wasi",
    target_os = "xous"
))]
// Consistent with `std`, `env_imp::Env` is `!Sync` in these platforms
impl !DynSync for std::env::VarsOs {}

macro_rules! already_sync {
    ($([$ty: ty])*) => {
        $(unsafe impl DynSync for $ty where Self: Sync {})*
    };
}

// Implements `DynSync` for types that are already `Sync`.
// This is a convenient way to bring existing `Sync` types into the `DynSync` ecosystem.
already_sync!(
    [std::sync::atomic::AtomicBool][std::sync::atomic::AtomicUsize][std::sync::atomic::AtomicU8]
        [std::sync::atomic::AtomicU32][std::backtrace::Backtrace][std::io::Error][std::fs::File][std::panic::Location<'_>]
        [jobserver_crate::Client][jobserver_crate::HelperThread][crate::memmap::Mmap]
        [crate::profiling::SelfProfiler][crate::owned_slice::OwnedSlice]
);

// Use portable AtomicU64 for targets without native 64-bit atomics
#[cfg(target_has_atomic = "64")]
already_sync!([std::sync::atomic::AtomicU64]);

#[cfg(not(target_has_atomic = "64"))]
already_sync!([portable_atomic::AtomicU64]);

macro_rules! impl_dyn_sync {
    ($($($attr: meta)* [$ty: ty where $($generics2: tt)*])*) => {
        $(unsafe impl<$($generics2)*> DynSync for $ty {})*
    };
}

// Explicitly implements `DynSync` for types that are known to be safe to share
// dynamically, often with specific generic bounds. This macro allows for
// fine-grained control over `DynSync` implementations for complex types.
impl_dyn_sync!(
    [std::sync::atomic::AtomicPtr<T> where T]
    [std::sync::OnceLock<T> where T: DynSend + DynSync]
    [std::sync::Mutex<T> where T: ?Sized + DynSend]
    [std::sync::Arc<T> where T: ?Sized + DynSync + DynSend]
    [std::sync::LazyLock<T, F> where T: DynSend + DynSync, F: DynSend]
    [std::collections::HashSet<K, S> where K: DynSync, S: DynSync]
    [std::collections::HashMap<K, V, S> where K: DynSync, V: DynSync, S: DynSync]
    [std::collections::BTreeMap<K, V, A> where K: DynSync, V: DynSync, A: std::alloc::Allocator + Clone + DynSync]
    [Vec<T, A> where T: DynSync, A: std::alloc::Allocator + DynSync]
    [Box<T, A> where T: ?Sized + DynSync, A: std::alloc::Allocator + DynSync]
    [crate::sync::RwLock<T> where T: DynSend + DynSync]
    [crate::sync::WorkerLocal<T> where T: DynSend]
    [crate::intern::Interned<'a, T> where 'a, T: DynSync]
    [crate::tagged_ptr::TaggedRef<'a, P, T> where 'a, P: Sync, T: Sync + crate::tagged_ptr::Tag]
    [parking_lot::lock_api::Mutex<R, T> where R: DynSync, T: ?Sized + DynSend]
    [parking_lot::lock_api::RwLock<R, T> where R: DynSync, T: ?Sized + DynSend + DynSync]
    [hashbrown::HashTable<T> where T: DynSync]
    [indexmap::IndexSet<V, S> where V: DynSync, S: DynSync]
    [indexmap::IndexMap<K, V, S> where K: DynSync, V: DynSync, S: DynSync]
    // // [smallvec::SmallVec<A> where A: smallvec::Array + DynSync]
    [thin_vec::ThinVec<T> where T: DynSync]
);

pub fn assert_dyn_sync<T: ?Sized + PointeeSized + DynSync>() {}
pub fn assert_dyn_send<T: ?Sized + PointeeSized + DynSend>() {}
pub fn assert_dyn_send_val<T: ?Sized + PointeeSized + DynSend>(_t: &T) {}
pub fn assert_dyn_send_sync_val<T: ?Sized + PointeeSized + DynSync + DynSend>(_t: &T) {}

/// A wrapper struct that allows dynamic control over `Send` and `Sync` properties
/// based on a runtime check. When `sync::is_dyn_thread_safe()` returns `true`,
/// types wrapped in `FromDyn` can be treated as `Send` and `Sync` if their
/// inner type `T` implements `DynSend` and `DynSync`, respectively.
#[derive(Copy, Clone)]
pub struct FromDyn<T>(T);

impl<T> FromDyn<T> {
    /// Creates a new `FromDyn` instance.
    ///
    /// This method asserts that `sync::is_dyn_thread_safe()` is true at the time
    /// of creation. This check ensures that it is safe to later implement `Send`
    /// and `Sync` for `FromDyn<T>` when `T` itself is `DynSend` or `DynSync`.
    #[inline(always)]
    pub fn from(val: T) -> Self {
        // Check that `sync::is_dyn_thread_safe()` is true on creation so we can
        // implement `Send` and `Sync` for this structure when `T`
        // implements `DynSend` and `DynSync` respectively.
        assert!(crate::sync::is_dyn_thread_safe());
        FromDyn(val)
    }

    /// Derives a new `FromDyn` instance for a different type `O`, leveraging
    /// the thread-safety check already performed during the creation of `self`.
    #[inline(always)]
    pub fn derive<O>(&self, val: O) -> FromDyn<O> {
        // We already did the check for `sync::is_dyn_thread_safe()` when creating `Self`
        FromDyn(val)
    }

    /// Consumes `self` and returns the inner value.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

// `FromDyn` is `Send` if `T` is `DynSend`, since it ensures that sync::is_dyn_thread_safe() is true.
unsafe impl<T: DynSend> Send for FromDyn<T> {}

// `FromDyn` is `Sync` if `T` is `DynSync`, since it ensures that sync::is_dyn_thread_safe() is true.
unsafe impl<T: DynSync> Sync for FromDyn<T> {}

impl<T> std::ops::Deref for FromDyn<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for FromDyn<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// A wrapper to convert a struct that is already a `Send` or `Sync` into
// an instance of `DynSend` and `DynSync`, since the compiler cannot infer
// it automatically in some cases. (e.g. Box<dyn Send / Sync>)
#[derive(Copy, Clone)]
pub struct IntoDynSyncSend<T: ?Sized + PointeeSized>(pub T);

// Implements `DynSend` for `IntoDynSyncSend<T>` if the inner type `T` is `Send`.
// This allows explicit marking of `Send` types as `DynSend`.
unsafe impl<T: ?Sized + PointeeSized + Send> DynSend for IntoDynSyncSend<T> {}

// Implements `DynSync` for `IntoDynSyncSend<T>` if the inner type `T` is `Sync`.
// This allows explicit marking of `Sync` types as `DynSync`.
unsafe impl<T: ?Sized + PointeeSized + Sync> DynSync for IntoDynSyncSend<T> {}

impl<T> std::ops::Deref for IntoDynSyncSend<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::DerefMut for IntoDynSyncSend<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
