use crate::BytesAtMost;
use byteorder::WriteBytesExt as _;
use std::io::{Cursor, Write as _};

pub struct ByteCode {
    pub prefix: Option<u8>,        // 0 ~ 1 byte
    pub rex: Option<Rex>,          // 0 ~ 1 byte
    pub opcode: BytesAtMost<3>,    // 1 ~ 3 byte
    pub mod_rm: Option<ModRM>,     // 0 ~ 1 byte
    pub sib: Option<Sib>,          // 0 ~ 1 byte
    pub addr_disp: BytesAtMost<4>, // 0 ~ 4 byte
    pub imm: BytesAtMost<4>,       // 0 ~ 4 byte
}

impl ByteCode {
    pub fn new() -> Self {
        ByteCode {
            prefix: None,
            rex: None,
            opcode: BytesAtMost::new(1),
            mod_rm: None,
            sib: None,
            addr_disp: BytesAtMost::new(0),
            imm: BytesAtMost::new(0),
        }
    }

    pub fn to_bytes(&self) -> BytesAtMost<15> {
        let len = self.prefix.is_some() as usize
            + self.rex.is_some() as usize
            + self.opcode.len()
            + self.mod_rm.is_some() as usize
            + self.sib.is_some() as usize
            + self.addr_disp.len()
            + self.imm.len();

        let mut bytes = BytesAtMost::new(len);

        let mut cursor = Cursor::new(bytes.bytes_mut());

        if let Some(prefix) = self.prefix {
            cursor.write_u8(prefix).unwrap();
        }

        if let Some(rex) = self.rex.as_ref() {
            cursor.write_u8(rex.byte()).unwrap();
        }

        cursor.write_all(self.opcode.bytes()).unwrap();

        if let Some(mod_rm) = self.mod_rm.as_ref() {
            cursor.write_u8(mod_rm.byte()).unwrap();
        }

        if let Some(sib) = self.sib.as_ref() {
            cursor.write_u8(sib.byte()).unwrap();
        }

        cursor.write_all(self.addr_disp.bytes()).unwrap();

        cursor.write_all(self.imm.bytes()).unwrap();

        bytes
    }
}

impl Default for ByteCode {
    fn default() -> Self {
        ByteCode::new()
    }
}

pub struct Rex(u8);

impl Rex {
    pub fn new() -> Self {
        Rex(0b0100_0000)
    }

    pub fn from_raw(raw: u8) -> Self {
        assert!(raw & 0b1111_0000 == 0b0100_0000);
        Rex(raw)
    }

    pub fn byte(&self) -> u8 {
        self.0
    }

    pub fn w(&self) -> bool {
        (self.0 & 0b0000_1000) != 0
    }

    pub fn set_w(&mut self, flag: bool) {
        if flag {
            self.0 |= 0b0000_1000;
        } else {
            self.0 &= 0b1111_0111;
        }
    }

    pub fn r(&self) -> bool {
        (self.0 & 0b0000_0100) != 0
    }

    pub fn set_r(&mut self, flag: bool) {
        if flag {
            self.0 |= 0b0000_0100;
        } else {
            self.0 &= 0b1111_1011;
        }
    }

    pub fn x(&self) -> bool {
        (self.0 & 0b0000_0010) != 0
    }

    pub fn set_x(&mut self, flag: bool) {
        if flag {
            self.0 |= 0b0000_0010;
        } else {
            self.0 &= 0b1111_1101;
        }
    }

    pub fn b(&self) -> bool {
        (self.0 & 0b0000_0001) != 0
    }

    pub fn set_b(&mut self, flag: bool) {
        if flag {
            self.0 |= 0b0000_0001;
        } else {
            self.0 &= 0b1111_1110;
        }
    }
}

pub struct ModRM(u8);

impl ModRM {
    pub fn new() -> ModRM {
        ModRM(0)
    }

    pub fn from_raw(raw: u8) -> Self {
        ModRM(raw)
    }

    pub fn byte(&self) -> u8 {
        self.0
    }

    pub fn mode(&self) -> u8 {
        self.0 >> 6
    }

    /// mode is 2 bits.
    pub fn set_mode(&mut self, mode: u8) {
        assert!(mode <= 0b11);
        self.0 = (self.0 & 0b00_111_111) + (mode << 6);
    }

    pub fn reg(&self) -> u8 {
        (self.0 & 0b00_111_000) >> 3
    }

    /// reg is 3 bits.
    pub fn set_reg(&mut self, reg: u8) {
        assert!(reg <= 0b111);
        self.0 = (self.0 & 0b11_000_111) + (reg << 3);
    }

    pub fn rm(&self) -> u8 {
        self.0 & 0b00000111
    }

    /// rm is 3 bits.
    pub fn set_rm(&mut self, rm: u8) {
        assert!(rm <= 0b111);
        self.0 = (self.0 & 0b11_111_000) + rm;
    }
}

pub struct Sib(u8);

impl Sib {
    pub fn new(scale: u8, index: u8, base: u8) -> Self {
        assert!(scale <= 0b11);
        assert!(index <= 0b111);
        assert!(base <= 0b111);

        Sib(scale << 6 | index << 3 | base)
    }

    pub fn from_raw(raw: u8) -> Self {
        Sib(raw)
    }

    pub fn byte(&self) -> u8 {
        self.0
    }

    pub fn scale(&self) -> u8 {
        self.0 >> 6
    }

    pub fn index(&self) -> u8 {
        (self.0 & 0b00111000) >> 3
    }

    pub fn base(&self) -> u8 {
        self.0 & 0b00000111
    }
}
