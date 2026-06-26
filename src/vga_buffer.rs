#![allow(dead_code)]

pub const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
pub const VGA_COLOR: u8 = 0x0F;

static mut CURRENT_COL: usize = 0;
static mut CURRENT_ROW: usize = 0;

pub unsafe fn clear_screen() {
    unsafe {
        for i in 0..(80 * 25) {
            *VGA_BUFFER.offset(i as isize * 2) = b' ';
            *VGA_BUFFER.offset(i as isize * 2 + 1) = VGA_COLOR;
        }
        CURRENT_COL = 0;
        CURRENT_ROW = 0;
    }
}

pub unsafe fn write_byte(byte: u8, color: u8) {
    unsafe {
        if byte == b'\n' {
            CURRENT_COL = 0;
            CURRENT_ROW += 1;
            return;
        }

        let offset = (CURRENT_ROW * 80 + CURRENT_COL) as isize;
        *VGA_BUFFER.offset(offset * 2) = byte;
        *VGA_BUFFER.offset(offset * 2 + 1) = color;

        CURRENT_COL += 1;
        if CURRENT_COL >= 80 {
            CURRENT_COL = 0;
            CURRENT_ROW += 1;
        }
    }
}

pub fn write_string(s: &str, color: u8) {
    unsafe {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => write_byte(byte, color),
                _ => write_byte(0xfe, color),
            }
        }
    }
}

pub unsafe fn write_line(s: &str, color: u8) {
    unsafe {
        write_string(s, color);
        write_byte(b'\n', color);
    }
}

pub unsafe fn backspace_on_vga() {
    unsafe {
        if CURRENT_COL > 0 {
            CURRENT_COL -= 1;
        } else {
            if CURRENT_ROW > 0 {
                CURRENT_ROW -= 1;
                CURRENT_COL = 79;
            }
        }
        let offset = (CURRENT_ROW * 80 + CURRENT_COL) as isize;
        *VGA_BUFFER.offset(offset * 2) = b' ';
        *VGA_BUFFER.offset(offset * 2 + 1) = VGA_COLOR;
    }
}