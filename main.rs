#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[unsafe(link_section = ".multiboot_header")]
#[unsafe(no_mangle)]
pub static MULTIBOOT_HEADER: [u32; 4] = [0x1badb002, 0x00, -(0x1badb002i32) as u32, 0];

// I/Oポート操作関数
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}

// PCスピーカー制御
unsafe fn play_sound(n_frequence: u32) {
    let div = 1193180 / n_frequence;
    outb(0x43, 0xb6);
    outb(0x42, (div & 0xff) as u8);
    outb(0x42, ((div >> 8) & 0xff) as u8);
    let tmp = inb(0x61);
    if tmp != (tmp | 3) { outb(0x61, tmp | 3); }
}

unsafe fn nosound() {
    let tmp = inb(0x61) & 0xfc;
    outb(0x61, tmp);
}

static mut SAVE_BUFFER: [u8; 4000] = [0; 4000];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut current_color: u8 = 0x0b; // 初期色：シアン
    let mut cursor_pos = 0;

    // 画面初期化：強力なクリア（黒背景に設定）
    unsafe {
        for i in 0..2000 {
            *vga_buffer.offset(i * 2) = 0x20;
            *vga_buffer.offset(i * 2 + 1) = 0x00;
        }
    }

    // スキャンコードマップ（修正済み：バックスラッシュの扱いを安全にしました）
    let scancode_map: [u8; 54] = [
        0,  27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8,
        9, b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'[', b']', 13,
        0, b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b';', b'\'', b'`',
        0, b'\\', b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b',', b'.', b'/',
    ];

    loop {
        unsafe {
            if inb(0x64) & 0x1 != 0 {
                let scancode = inb(0x60);
                if scancode > 0 && scancode < 0x80 {
                    match scancode {
                        0x3b => { // F1: Save
                            for i in 0..4000 { SAVE_BUFFER[i] = *vga_buffer.offset(i as isize); }
                        },
                        0x3c => { // F2: Clear
                            for i in 0..2000 {
                                *vga_buffer.offset(i * 2) = 0x20;
                                *vga_buffer.offset(i * 2 + 1) = 0x00;
                            }
                            cursor_pos = 0;
                        },
                        0x3d => { // F3: Load
                            for i in 0..4000 { *vga_buffer.offset(i as isize) = SAVE_BUFFER[i]; }
                        },
                        0x3e => { // F4: 色変更
                            current_color = (current_color + 1) % 16;
                            if current_color == 0 { current_color = 1; }
                            for i in 0..2000 { *vga_buffer.offset(i * 2 + 1) = current_color; }
                        },
                        0x3f => play_sound(440), // F5: 音
                        0x40 => nosound(),        // F6: 止める
                        _ => {
                            if (scancode as usize) < scancode_map.len() {
                                let ascii = scancode_map[scancode as usize];
                                if ascii >= 32 {
                                    *vga_buffer.offset(cursor_pos * 2) = ascii;
                                    *vga_buffer.offset(cursor_pos * 2 + 1) = current_color;
                                    cursor_pos += 1;
                                    for _ in 0..100000 { asm!("nop"); }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}