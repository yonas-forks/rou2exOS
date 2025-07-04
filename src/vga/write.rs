use crate::vga::buffer;
use crate::vga::screen;

pub fn number(vga_index: &mut isize, num: u64) {
    let mut buf = [0u8; 20];
    let mut i = buf.len();

    if num == 0 {
        string(vga_index, b"0", buffer::Color::White);
        return;
    }

    let mut n = num;

    while n > 0 {
        i -= 1;
        if let Some(b) = buf.get_mut(i) {
            *b = b'0' + (n % 10) as u8;
        }
        n /= 10;
    }

    string(vga_index, buf.get(i..).unwrap_or(&[]) as &[u8], buffer::Color::White);
}

/// Write a whole string to screen
pub fn string(vga_index: &mut isize, string: &[u8], color: buffer::Color) {
    screen::scroll(vga_index);

    for &b in string {
        if b == b'\n' {
            newline(vga_index);
            continue;
        }

        byte(vga_index, b, color);
    }
}

pub fn byte(vga_index: &mut isize, b: u8, color: buffer::Color) {
    unsafe {
        *buffer::VGA_BUFFER.offset(*vga_index) = b;
        *buffer::VGA_BUFFER.offset(*vga_index + 1) = color as u8;
        *vga_index += 2;
    }
}

/// Move to a new line
pub fn byte_raw(vga_index: &mut isize, b: u8, color: u8) {
    unsafe {
        *buffer::VGA_BUFFER.offset(*vga_index) = b;
        *buffer::VGA_BUFFER.offset(*vga_index + 1) = color;
        *vga_index += 2;
    }
}

pub fn newline(vga_index: &mut isize) {
    // VGA 80x25: each line is 80 chars * 2 bytes per char
    *vga_index += (80 * 2) - (*vga_index % (80 * 2));
}


