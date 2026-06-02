// Copyright 2025 The safe-mmio Authors.
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Default MMIO backend using volatile reads and writes.

use core::ptr;

#[unsafe(no_mangle)]
unsafe extern "Rust" fn __safe_mmio_read(src: *const u8, dst: *mut u8, len: usize) {
    // SAFETY: The caller guarantees src is a valid MMIO address and dst has room for len bytes.
    unsafe {
        match len {
            1 => ptr::write(dst, ptr::read_volatile(src)),
            2 => ptr::write(dst.cast::<u16>(), ptr::read_volatile(src.cast::<u16>())),
            4 => ptr::write(dst.cast::<u32>(), ptr::read_volatile(src.cast::<u32>())),
            8 => ptr::write(dst.cast::<u64>(), ptr::read_volatile(src.cast::<u64>())),
            _ => {
                let mut offset = 0;
                while offset + 8 <= len {
                    ptr::write(
                        dst.add(offset).cast::<u64>(),
                        ptr::read_volatile(src.add(offset).cast::<u64>()),
                    );
                    offset += 8;
                }
                while offset + 4 <= len {
                    ptr::write(
                        dst.add(offset).cast::<u32>(),
                        ptr::read_volatile(src.add(offset).cast::<u32>()),
                    );
                    offset += 4;
                }
                while offset + 2 <= len {
                    ptr::write(
                        dst.add(offset).cast::<u16>(),
                        ptr::read_volatile(src.add(offset).cast::<u16>()),
                    );
                    offset += 2;
                }
                if offset < len {
                    ptr::write(dst.add(offset), ptr::read_volatile(src.add(offset)));
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "Rust" fn __safe_mmio_write(dst: *mut u8, src: *const u8, len: usize) {
    // SAFETY: The caller guarantees dst is a valid MMIO address and src has len bytes.
    unsafe {
        match len {
            1 => ptr::write_volatile(dst, ptr::read(src)),
            2 => ptr::write_volatile(dst.cast::<u16>(), ptr::read(src.cast::<u16>())),
            4 => ptr::write_volatile(dst.cast::<u32>(), ptr::read(src.cast::<u32>())),
            8 => ptr::write_volatile(dst.cast::<u64>(), ptr::read(src.cast::<u64>())),
            _ => {
                let mut offset = 0;
                while offset + 8 <= len {
                    ptr::write_volatile(
                        dst.add(offset).cast::<u64>(),
                        ptr::read(src.add(offset).cast::<u64>()),
                    );
                    offset += 8;
                }
                while offset + 4 <= len {
                    ptr::write_volatile(
                        dst.add(offset).cast::<u32>(),
                        ptr::read(src.add(offset).cast::<u32>()),
                    );
                    offset += 4;
                }
                while offset + 2 <= len {
                    ptr::write_volatile(
                        dst.add(offset).cast::<u16>(),
                        ptr::read(src.add(offset).cast::<u16>()),
                    );
                    offset += 2;
                }
                if offset < len {
                    ptr::write_volatile(dst.add(offset), ptr::read(src.add(offset)));
                }
            }
        }
    }
}
