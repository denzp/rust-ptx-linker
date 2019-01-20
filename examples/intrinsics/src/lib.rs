#![deny(warnings)]
#![feature(abi_ptx, stdsimd)]
#![no_std]

use core::arch::nvptx::*;

mod image;
use image::{Image, InputPixel, MutImage, OutputPixel};

extern "C" {
    pub fn vprintf(format: *const u8, valist: *const u8) -> i32;
    pub fn malloc(size: u64) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn rgb2gray(
    src: *const InputPixel,
    dst: *mut OutputPixel,
    width: u32,
) {
    let i = (_block_dim_y() * _block_idx_y() + _thread_idx_y()) as i32;
    let j = (_block_dim_x() * _block_idx_x() + _thread_idx_x()) as i32;

    let src_image = Image::<InputPixel> {
        pixels: src,
        width: width as i32,
    };

    let mut dst_image = MutImage::<OutputPixel> {
        pixels: dst,
        width: width as i32,
    };

    let mut accumulator: u16 = 0;

    accumulator += src_image.pixel(i, j).r as u16;
    accumulator += src_image.pixel(i, j).g as u16;
    accumulator += src_image.pixel(i, j).b as u16;

    dst_image.mut_pixel(i, j).l = (accumulator / 3) as u8;
}

#[no_mangle]
pub unsafe extern "ptx-kernel" fn syscalls_kernel() {
    vprintf("allocating memory".as_ptr(), [].as_ptr());
    let ptr = malloc(32);

    vprintf("writing into the memory".as_ptr(), [].as_ptr());
    *ptr.offset(0) = 128;

    vprintf("releasing memory".as_ptr(), [].as_ptr());
    free(ptr);
}

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
