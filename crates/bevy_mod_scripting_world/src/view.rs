//! Views: addresses of components and resources in the world, which a language reads and writes in place (LuaJIT).
//!
//! A view is an address plus the two epochs it was made in. It may be used only while both are unchanged:
//! - the [structural epoch](structural_epoch), one for the process, is bumped by every structural change made through a
//!   [`WorldAccessGuard`](crate::WorldAccessGuard) (`with_world_mut_access`): columns may have moved, rows been swapped.
//!   Structural changes need exclusive access to the world, so no script runs (and reads it) while it changes;
//! - the [context's epoch](ViewEpoch), bumped when the context's script code is entered and left: the world may have
//!   changed in any way in between, and Rust code must not keep a borrow of viewed data across a call into a script.
//!
//! Lua never holds a Rust reference: only languages checking both epochs before every access may be given an address
//! (see [`crate::WorldAccessGuard::row_view`]).

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// `1.0`, the first epoch
const ONE: u64 = 0x3FF0_0000_0000_0000;

/// See the module docs. An `f64` in its bits, so languages read it as a number: exact up to 2^53 bumps.
static STRUCTURAL: AtomicU64 = AtomicU64::new(ONE);

fn bump(cell: &AtomicU64) {
    let next = f64::from_bits(cell.load(Ordering::Relaxed)) + 1.0;
    cell.store(next.to_bits(), Ordering::Relaxed);
}

/// Makes every view stale: called before any structural change.
pub fn bump_structural_epoch() {
    bump(&STRUCTURAL)
}

/// The structural epoch.
pub fn structural_epoch_value() -> f64 {
    f64::from_bits(STRUCTURAL.load(Ordering::Relaxed))
}

/// The address of the structural epoch, an `f64`, for languages to read.
pub fn structural_epoch() -> *const f64 {
    STRUCTURAL.as_ptr().cast_const().cast()
}

/// The epoch of one script context (i.e. a Lua state), see the module docs, and the change tick its views write.
/// Clones are the same epoch.
///
/// Its addresses are stable while it lives: the language is given them once. Only the thread running the context uses
/// it.
#[derive(Clone, Debug)]
pub struct ViewEpoch(std::sync::Arc<ViewCells>);

std::thread_local! {
    /// The epoch of the context whose script code runs on this thread, see [`ViewEpoch::enter`]
    static CURRENT: std::cell::RefCell<Option<ViewEpoch>> = const { std::cell::RefCell::new(None) };
}

#[derive(Debug)]
#[repr(C)]
struct ViewCells {
    /// An `f64` in its bits, see [`STRUCTURAL`]
    epoch: AtomicU64,
    /// The `this_run` tick of the views made last
    this_run: AtomicU32,
}

impl Default for ViewEpoch {
    fn default() -> Self {
        Self(std::sync::Arc::new(ViewCells {
            epoch: AtomicU64::new(ONE),
            this_run: AtomicU32::new(0),
        }))
    }
}

impl ViewEpoch {
    /// Makes the views of this context stale.
    pub fn bump(&self) {
        bump(&self.0.epoch)
    }

    /// The epoch.
    pub fn get(&self) -> f64 {
        f64::from_bits(self.0.epoch.load(Ordering::Relaxed))
    }

    /// The address of the epoch, an `f64`.
    pub fn as_ptr(&self) -> *const f64 {
        self.0.epoch.as_ptr().cast_const().cast()
    }

    /// The address of the change tick views write into the rows they change, a `u32`.
    pub fn this_run_ptr(&self) -> *const u32 {
        self.0.this_run.as_ptr().cast_const()
    }

    /// Runs `f` as script code of this context: views made before are stale in `f`, views made in `f` are stale after
    /// it, also when it panics. The context is the [current one](Self::current) during `f`: views are only made for it.
    pub fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        struct Leave<'a>(&'a ViewEpoch, Option<ViewEpoch>);
        impl Drop for Leave<'_> {
            fn drop(&mut self) {
                self.0.bump();
                let outer = self.1.take();
                CURRENT.with(|current| *current.borrow_mut() = outer);
            }
        }
        self.bump();
        let outer = CURRENT.with(|current| current.borrow_mut().replace(self.clone()));
        let _leave = Leave(self, outer);
        f()
    }

    /// The epoch of the context whose script code runs on this thread (the innermost [`Self::enter`]), if any.
    pub fn current() -> Option<ViewEpoch> {
        CURRENT.with(|current| current.borrow().clone())
    }

    /// The address standing for this epoch, the same as [`Self::as_ptr`], see [`Self::from_raw`].
    pub fn as_raw(&self) -> *const std::ffi::c_void {
        std::sync::Arc::as_ptr(&self.0).cast()
    }

    /// The epoch of an address from [`Self::as_raw`].
    ///
    /// # Safety
    /// `raw` is from [`Self::as_raw`] of an epoch which is alive.
    pub unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self {
        let raw = raw.cast::<ViewCells>();
        // Safety: an epoch is alive, which holds a count: the new one is owned by the result
        unsafe {
            std::sync::Arc::increment_strong_count(raw);
            Self(std::sync::Arc::from_raw(raw))
        }
    }

    /// Whether both are the epoch of the same context.
    pub fn same(&self, other: &ViewEpoch) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }

    pub(crate) fn set_this_run(&self, tick: u32) {
        self.0.this_run.store(tick, Ordering::Relaxed)
    }
}

/// A component or resource handed out as a view, see [`crate::WorldAccessGuard::row_view`].
#[derive(Clone, Copy, Debug)]
pub struct RowView {
    /// The value
    pub data: *mut u8,
    /// Its changed tick, a `u32`: views write their context's `this_run` there after each write. Null if read-only.
    pub changed_tick: *mut u32,
    /// The context's epoch when it was made
    pub epoch: f64,
    /// The structural epoch when it was made
    pub structural: f64,
}

// views write `this_run` into a `Tick` as a `u32`
const _: () = assert!(
    size_of::<bevy_ecs::change_detection::Tick>() == size_of::<u32>()
        && align_of::<bevy_ecs::change_detection::Tick>() == align_of::<u32>()
);
