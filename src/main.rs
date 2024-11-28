#![no_std]
#![no_main]

use core::arch::{asm, global_asm};

#[inline(always)]
fn print_char(ch: u8) {
    unsafe {
        asm!("int 0x10",
            in("ah") 0x0e_u8,
            in("al") ch,
            in("bl") 0xff_u8,
        );
    }
}

#[no_mangle]
#[naked_function::naked]
unsafe extern "C" fn _start() -> ! {
    asm!("
        mov sp, 0x7c00
        jmp rust_entry
        cli
        hlt
    ");
}

#[inline(always)]
fn never() -> ! {
    unsafe {
        asm!("
            cli
            hlt
        ", options(noreturn, nomem))
    }
}

#[no_mangle]
unsafe extern "C" fn rust_entry() -> ! {
    for c in *b"Hello, world!" {
        print_char(c);
    }

    never()
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    never()
}
