// Copyright 2025 The safe-mmio Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use crate::{SharedMmioPointer, UniqueMmioPointer};
use core::mem::size_of;
use zerocopy::{FromBytes, Immutable, IntoBytes};

unsafe extern "Rust" {
    fn __safe_mmio_read(src: *const u8, dst: *mut u8, len: usize);
    fn __safe_mmio_write(dst: *mut u8, src: *const u8, len: usize);
}

impl<T: FromBytes + IntoBytes> UniqueMmioPointer<'_, T> {
    /// Performs an MMIO read of the entire `T`.
    ///
    /// # Safety
    ///
    /// This field must be safe to perform an MMIO read from.
    pub unsafe fn read_unsafe(&mut self) -> T {
        let mut value = T::new_zeroed();
        // SAFETY: self.regs is always a valid and unique pointer to MMIO address space.
        // The caller guarantees the field is safe to read.
        unsafe {
            __safe_mmio_read(
                self.regs.as_ptr().cast::<u8>().cast_const(),
                value.as_mut_bytes().as_mut_ptr(),
                size_of::<T>(),
            );
        }
        value
    }
}

impl<T: Immutable + IntoBytes> UniqueMmioPointer<'_, T> {
    /// Performs an MMIO write of the entire `T`.
    ///
    /// # Safety
    ///
    /// This field must be safe to perform an MMIO write to.
    pub unsafe fn write_unsafe(&self, value: T) {
        // SAFETY: self.regs is always a valid and unique pointer to MMIO address space.
        // The caller guarantees the field is safe to write.
        unsafe {
            __safe_mmio_write(
                self.regs.as_ptr().cast::<u8>(),
                value.as_bytes().as_ptr(),
                size_of::<T>(),
            );
        }
    }
}

impl<T: FromBytes + IntoBytes> SharedMmioPointer<'_, T> {
    /// Performs an MMIO read of the entire `T`.
    ///
    /// # Safety
    ///
    /// This field must be safe to perform an MMIO read from, and doing so must not cause any
    /// side-effects.
    pub unsafe fn read_unsafe(&self) -> T {
        let mut value = T::new_zeroed();
        // SAFETY: self.regs is always a valid pointer to MMIO address space.
        // The caller guarantees the field is safe to read without side-effects.
        unsafe {
            __safe_mmio_read(
                self.regs.as_ptr().cast::<u8>().cast_const(),
                value.as_mut_bytes().as_mut_ptr(),
                size_of::<T>(),
            );
        }
        value
    }
}
