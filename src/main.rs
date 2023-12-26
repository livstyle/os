//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]

use core::{panic::PanicInfo, arch::{global_asm}};


const UART: usize = 0x1000_0000;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));


/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn console_putchar(bytes: &[u8]) {
    if let [head, tail @ ..] = bytes {
        unsafe { (UART as *mut u8).write_volatile(*head) };
        console_putchar(tail);
    }
}

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 在屏幕上输出 "Hello World!!!\n" ，随后进入死循环
    console_putchar(b"Hello World!!!\n");

    loop {}
}