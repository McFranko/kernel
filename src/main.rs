#![allow(non_snake_case)]
#![no_std]
#![no_main]
#![feature(asm)]

pub mod panic;
pub mod processes;
pub mod kout;

use kout::Kout;

pub const VGA_BUFFER_WIDTH: u8 = 80;
pub const VGA_BUFFER_HEIGHT: u8 = 20;

#[no_mangle]
fn _start() -> ! {
    let mut writer = Kout {
        columnPosition: 0
    };
    writer.writeString("Starting kernel\n");
    loop {}
}
