use crate::FlexBytes;
use byteorder::WriteBytesExt as _;
use std::io::{Cursor, Write as _};

pub struct ByteCode {
    pub prefix: Option<u8>,    // 0 ~ 1 byte
    pub rex: Option<Rex>,      // 0 ~ 1 byte
    pub opcode: FlexBytes<3>,  // 1 ~ 3 byte
    pub mod_rm: Option<ModRM>, // 0 ~ 1 byte
    pub sib: Option<Sib>,      // 0 ~ 1 byte
    pub addr: FlexBytes<4>,    // 0 ~ 4 byte
    pub imm: FlexBytes<4>,     // 0 ~ 4 byte
}

impl ByteCode {
    pub fn new() -> Self {
        ByteCode {
            prefix: None,
            rex: None,
            opcode: FlexBytes::new(1),
            mod_rm: None,
            sib: None,
            addr: FlexBytes::new(0),
            imm: FlexBytes::new(0),
        }
    }

    pub fn to_bytes(&self) -> FlexBytes<15> {
        let len = self.prefix.is_some() as usize
            + self.rex.is_some() as usize
            + self.opcode.len()
            + self.mod_rm.is_some() as usize
            + self.sib.is_some() as usize
            + self.addr.len()
            + self.imm.len();

        let mut bytes = FlexBytes::new(len);

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

        cursor.write_all(self.addr.bytes()).unwrap();

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
    pub fn new(w: bool, r: bool, x: bool, b: bool) -> Self {
        Rex(0b0100_0000 | (w as u8) << 3 | (r as u8) << 2 | (x as u8) << 1 | (b as u8))
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

    pub fn r(&self) -> bool {
        (self.0 & 0b0000_0100) != 0
    }

    pub fn x(&self) -> bool {
        (self.0 & 0b0000_0010) != 0
    }

    pub fn b(&self) -> bool {
        (self.0 & 0b0000_0001) != 0
    }
}

pub struct ModRM(u8);

impl ModRM {
    pub fn new(mode: u8, reg: u8, r_m: u8) -> ModRM {
        assert!(mode <= 0b11);
        assert!(reg <= 0b111);
        assert!(r_m <= 0b111);

        ModRM(mode << 6 | reg << 3 | r_m)
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

    pub fn reg(&self) -> u8 {
        (self.0 & 0b00111000) >> 3
    }

    pub fn r_m(&self) -> u8 {
        self.0 & 0b00000111
    }
}

pub struct Sib(u8);

impl Sib {
    pub fn new(scale: u8, index: u8, base: u8) -> Self {
        assert!(scale <= 0b11);
        assert!(index <= 0b111);
        assert!(base <= 0b111);

        Sib(scale << 6 & index << 3 & base)
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