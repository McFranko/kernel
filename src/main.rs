#![allow(non_snake_case)]
#![no_std]
#![no_main]
#![feature(asm)]

pub mod kout;
pub mod panic;
pub mod processes;

use kout::Kout;

#[no_mangle]
fn _start() -> ! {
    let mut writer = Kout { columnPosition: 0 };
    writer.writeString("Starting kernel\n");
    loop {}
}
