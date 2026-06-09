// Copyright 2026 The safe-mmio Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Custom MMIO backend using consumer-provided read/write implementations.
//!
//! When the `custom-mmio` feature is enabled, the consumer must provide an implementation of the
//! [`MmioOps`] trait and register it using the [`set_mmio_ops!`](crate::set_mmio_ops) macro.
//! Linking will fail if no implementation is registered.
//!
//! # Example
//!
//! ```ignore
//! struct MyMmioBackend;
//!
//! unsafe impl safe_mmio::custom_mmio::MmioOps for MyMmioBackend {
//!     unsafe fn read_u8(src: *const u8) -> u8 { /* ... */ }
//!     unsafe fn read_u16(src: *const u16) -> u16 { /* ... */ }
//!     unsafe fn read_u32(src: *const u32) -> u32 { /* ... */ }
//!     unsafe fn read_u64(src: *const u64) -> u64 { /* ... */ }
//!     unsafe fn write_u8(dst: *mut u8, value: u8) { /* ... */ }
//!     unsafe fn write_u16(dst: *mut u16, value: u16) { /* ... */ }
//!     unsafe fn write_u32(dst: *mut u32, value: u32) { /* ... */ }
//!     unsafe fn write_u64(dst: *mut u64, value: u64) { /* ... */ }
//! }
//!
//! safe_mmio::set_mmio_ops!(MyMmioBackend);
//! ```

use crate::{SharedMmioPointer, UniqueMmioPointer};
use core::mem::{MaybeUninit, size_of};
use core::ptr::NonNull;

/// Trait for custom MMIO read/write implementations.
///
/// Per-size methods are used because MMIO access width matters at the hardware level (e.g. the
/// GHCB protocol needs to know the exact access size for VMGEXIT calls).
///
/// # Safety
///
/// Implementations must perform a single MMIO access of the indicated width at the given address.
/// The pointer is guaranteed to be properly aligned for its type and to point to valid MMIO address
/// space.
pub unsafe trait MmioOps {
    /// Perform an 8-bit MMIO read.
    ///
    /// # Safety
    ///
    /// `src` must be a valid, aligned pointer to MMIO address space.
    unsafe fn read_u8(src: *const u8) -> u8;

    /// Perform a 16-bit MMIO read.
    ///
    /// # Safety
    ///
    /// `src` must be a valid, aligned pointer to MMIO address space.
    unsafe fn read_u16(src: *const u16) -> u16;

    /// Perform a 32-bit MMIO read.
    ///
    /// # Safety
    ///
    /// `src` must be a valid, aligned pointer to MMIO address space.
    unsafe fn read_u32(src: *const u32) -> u32;

    /// Perform a 64-bit MMIO read.
    ///
    /// # Safety
    ///
    /// `src` must be a valid, aligned pointer to MMIO address space.
    unsafe fn read_u64(src: *const u64) -> u64;

    /// Perform an 8-bit MMIO write.
    ///
    /// # Safety
    ///
    /// `dst` must be a valid, aligned pointer to MMIO address space.
    unsafe fn write_u8(dst: *mut u8, value: u8);

    /// Perform a 16-bit MMIO write.
    ///
    /// # Safety
    ///
    /// `dst` must be a valid, aligned pointer to MMIO address space.
    unsafe fn write_u16(dst: *mut u16, value: u16);

    /// Perform a 32-bit MMIO write.
    ///
    /// # Safety
    ///
    /// `dst` must be a valid, aligned pointer to MMIO address space.
    unsafe fn write_u32(dst: *mut u32, value: u32);

    /// Perform a 64-bit MMIO write.
    ///
    /// # Safety
    ///
    /// `dst` must be a valid, aligned pointer to MMIO address space.
    unsafe fn write_u64(dst: *mut u64, value: u64);
}

unsafe extern "Rust" {
    fn __safe_mmio_read_u8(src: *const u8) -> u8;
    fn __safe_mmio_read_u16(src: *const u16) -> u16;
    fn __safe_mmio_read_u32(src: *const u32) -> u32;
    fn __safe_mmio_read_u64(src: *const u64) -> u64;
    fn __safe_mmio_write_u8(dst: *mut u8, value: u8);
    fn __safe_mmio_write_u16(dst: *mut u16, value: u16);
    fn __safe_mmio_write_u32(dst: *mut u32, value: u32);
    fn __safe_mmio_write_u64(dst: *mut u64, value: u64);
}

/// Register a [`MmioOps`] implementation as the MMIO backend.
///
/// This macro must be called exactly once in the final binary when the `custom-mmio` feature is
/// enabled. It generates the linker symbols that bridge the [`MmioOps`] trait to the internal
/// extern function declarations.
///
/// # Example
///
/// ```ignore
/// safe_mmio::set_mmio_ops!(MyMmioBackend);
/// ```
#[macro_export]
macro_rules! set_mmio_ops {
    ($t:ty) => {
        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_read_u8(src: *const u8) -> u8 {
            // SAFETY: Caller guarantees src is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::read_u8(src) }
        }

        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_read_u16(src: *const u16) -> u16 {
            // SAFETY: Caller guarantees src is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::read_u16(src) }
        }

        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_read_u32(src: *const u32) -> u32 {
            // SAFETY: Caller guarantees src is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::read_u32(src) }
        }

        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_read_u64(src: *const u64) -> u64 {
            // SAFETY: Caller guarantees src is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::read_u64(src) }
        }

        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_write_u8(dst: *mut u8, value: u8) {
            // SAFETY: Caller guarantees dst is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::write_u8(dst, value) }
        }

        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_write_u16(dst: *mut u16, value: u16) {
            // SAFETY: Caller guarantees dst is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::write_u16(dst, value) }
        }

        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_write_u32(dst: *mut u32, value: u32) {
            // SAFETY: Caller guarantees dst is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::write_u32(dst, value) }
        }

        #[unsafe(no_mangle)]
        unsafe fn __safe_mmio_write_u64(dst: *mut u64, value: u64) {
            // SAFETY: Caller guarantees dst is valid and aligned for MMIO.
            unsafe { <$t as $crate::custom_mmio::MmioOps>::write_u64(dst, value) }
        }
    };
}

/// Performs an MMIO read and returns the value.
///
/// # Safety
///
/// The pointer must be valid to perform an MMIO read from.
unsafe fn mmio_read<T>(ptr: NonNull<T>) -> T {
    // SAFETY: ptr is a valid, aligned pointer to MMIO address space. The extern functions are
    // provided by the consumer via set_mmio_ops!(). For sizes 1/2/4/8 we perform a single
    // access; for larger sizes we split into chunks. The MaybeUninit is fully initialized before
    // calling assume_init().
    unsafe {
        match size_of::<T>() {
            1 => {
                let val = __safe_mmio_read_u8(ptr.cast().as_ptr());
                let mut result = MaybeUninit::<T>::uninit();
                result.as_mut_ptr().cast::<u8>().write(val);
                result.assume_init()
            }
            2 => {
                let val = __safe_mmio_read_u16(ptr.cast().as_ptr());
                let mut result = MaybeUninit::<T>::uninit();
                result.as_mut_ptr().cast::<u16>().write(val);
                result.assume_init()
            }
            4 => {
                let val = __safe_mmio_read_u32(ptr.cast().as_ptr());
                let mut result = MaybeUninit::<T>::uninit();
                result.as_mut_ptr().cast::<u32>().write(val);
                result.assume_init()
            }
            8 => {
                let val = __safe_mmio_read_u64(ptr.cast().as_ptr());
                let mut result = MaybeUninit::<T>::uninit();
                result.as_mut_ptr().cast::<u64>().write(val);
                result.assume_init()
            }
            size => {
                let mut result = MaybeUninit::<T>::uninit();
                read_chunks(ptr.cast(), result.as_mut_ptr().cast(), size);
                result.assume_init()
            }
        }
    }
}

/// Reads `remaining` bytes from MMIO at `src` into `dst`, splitting into the largest possible
/// access widths.
///
/// # Safety
///
/// `src` must be a valid pointer to MMIO address space. `dst` must be valid for `remaining`
/// bytes. Both pointers must be properly aligned for 8-byte accesses if `remaining >= 8`.
unsafe fn read_chunks(mut src: NonNull<u8>, mut dst: *mut u8, mut remaining: usize) {
    // SAFETY: src points to valid MMIO address space and dst is valid for `remaining` bytes.
    // We advance both pointers by the access size after each operation, maintaining alignment
    // and staying within bounds.
    unsafe {
        while remaining >= 8 {
            dst.cast::<u64>()
                .write_unaligned(__safe_mmio_read_u64(src.cast().as_ptr()));
            src = NonNull::new_unchecked(src.as_ptr().add(8));
            dst = dst.add(8);
            remaining -= 8;
        }
        if remaining >= 4 {
            dst.cast::<u32>()
                .write_unaligned(__safe_mmio_read_u32(src.cast().as_ptr()));
            src = NonNull::new_unchecked(src.as_ptr().add(4));
            dst = dst.add(4);
            remaining -= 4;
        }
        if remaining >= 2 {
            dst.cast::<u16>()
                .write_unaligned(__safe_mmio_read_u16(src.cast().as_ptr()));
            src = NonNull::new_unchecked(src.as_ptr().add(2));
            dst = dst.add(2);
            remaining -= 2;
        }
        if remaining >= 1 {
            dst.write(__safe_mmio_read_u8(src.as_ptr()));
        }
    }
}

/// Writes `remaining` bytes from `src` to MMIO at `dst`, splitting into the largest possible
/// access widths.
///
/// # Safety
///
/// `dst` must be a valid pointer to MMIO address space. `src` must be valid for `remaining`
/// bytes. Both pointers must be properly aligned for 8-byte accesses if `remaining >= 8`.
unsafe fn write_chunks(mut dst: NonNull<u8>, mut src: *const u8, mut remaining: usize) {
    // SAFETY: dst points to valid MMIO address space and src is valid for `remaining` bytes.
    // We advance both pointers by the access size after each operation, maintaining alignment
    // and staying within bounds.
    unsafe {
        while remaining >= 8 {
            __safe_mmio_write_u64(dst.cast().as_ptr(), src.cast::<u64>().read_unaligned());
            dst = NonNull::new_unchecked(dst.as_ptr().add(8));
            src = src.add(8);
            remaining -= 8;
        }
        if remaining >= 4 {
            __safe_mmio_write_u32(dst.cast().as_ptr(), src.cast::<u32>().read_unaligned());
            dst = NonNull::new_unchecked(dst.as_ptr().add(4));
            src = src.add(4);
            remaining -= 4;
        }
        if remaining >= 2 {
            __safe_mmio_write_u16(dst.cast().as_ptr(), src.cast::<u16>().read_unaligned());
            dst = NonNull::new_unchecked(dst.as_ptr().add(2));
            src = src.add(2);
            remaining -= 2;
        }
        if remaining >= 1 {
            __safe_mmio_write_u8(dst.as_ptr(), src.read());
        }
    }
}

impl<T> UniqueMmioPointer<'_, T> {
    /// Performs an MMIO read of the entire `T`.
    ///
    /// Note that this takes `&mut self` rather than `&self` because an MMIO read may cause
    /// side-effects that change the state of the device.
    ///
    /// # Safety
    ///
    /// This field must be safe to perform an MMIO read from.
    pub unsafe fn read_unsafe(&mut self) -> T {
        // SAFETY: self.regs is always a valid and unique pointer to MMIO address space.
        unsafe { mmio_read(self.regs) }
    }

    /// Performs an MMIO write of the entire `T`.
    ///
    /// # Safety
    ///
    /// This field must be safe to perform an MMIO write to.
    pub unsafe fn write_unsafe(&mut self, value: T) {
        // SAFETY: self.regs is always a valid and unique pointer to MMIO address space.
        // We cast the value to its byte representation and dispatch to the appropriate
        // sized write operation.
        unsafe {
            match size_of::<T>() {
                1 => __safe_mmio_write_u8(
                    self.regs.cast().as_ptr(),
                    *(&value as *const T).cast::<u8>(),
                ),
                2 => __safe_mmio_write_u16(
                    self.regs.cast().as_ptr(),
                    (&value as *const T).cast::<u16>().read_unaligned(),
                ),
                4 => __safe_mmio_write_u32(
                    self.regs.cast().as_ptr(),
                    (&value as *const T).cast::<u32>().read_unaligned(),
                ),
                8 => __safe_mmio_write_u64(
                    self.regs.cast().as_ptr(),
                    (&value as *const T).cast::<u64>().read_unaligned(),
                ),
                size => write_chunks(self.regs.cast(), (&value as *const T).cast::<u8>(), size),
            }
        }
    }
}

impl<T> SharedMmioPointer<'_, T> {
    /// Performs an MMIO read of the entire `T`.
    ///
    /// # Safety
    ///
    /// This field must be safe to perform an MMIO read from, and doing so must not cause any
    /// side-effects.
    pub unsafe fn read_unsafe(&self) -> T {
        // SAFETY: self.regs is always a valid pointer to MMIO address space.
        unsafe { mmio_read(self.regs) }
    }
}
