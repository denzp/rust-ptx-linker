#![deny(warnings)]
#![feature(abi_ptx, panic_handler)]
#![no_std]

extern crate nvptx_builtins;
use nvptx_builtins::*;

mod image;
use image::{Image, InputPixel, MutImage, OutputPixel};

#[no_mangle]
pub unsafe extern "ptx-kernel" fn rgb2gray(
    src: *const InputPixel,
    dst: *mut OutputPixel,
    width: u32,
) {
    let i = (block_dim_y() * block_idx_y() + thread_idx_y()) as i32;
    let j = (block_dim_x() * block_idx_x() + thread_idx_x()) as i32;

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

#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}
