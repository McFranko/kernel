#![allow(non_snake_case)]

pub struct Kout {
    pub columnPosition: u8,
}

impl Kout {
    pub fn writeByte(&mut self, byte: u8) {
        let vgaBuffer = 0xb8000 as *mut u8;
        match byte {
            b'\n' => self.newLine(),
            _ => unsafe {
                *vgaBuffer.offset(self.columnPosition as isize * 2) = byte;
                *vgaBuffer.offset(self.columnPosition as isize * 2 + 1) = 0x0f;
            },
        }
        self.columnPosition += 1;
    }

    pub fn writeString(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.writeByte(byte),
                _ => self.writeByte(0xfe),
            }
        }
    }
    fn newLine(&mut self) {
        for position in self.columnPosition..80 {
            self.writeByte(b' ');
        }
    }
}
