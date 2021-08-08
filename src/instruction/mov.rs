use crate::{ByteCode, FlexBytes, Mem64, ModRM, Reg64, Rex};
use byteorder::{WriteBytesExt as _, LE};

pub struct Mov<Dst, Src>(Dst, Src);

impl Mov<Mem64, Reg64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // set REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_r(src.rex_r());
        rex.set_x(dst.rex_x());
        rex.set_b(dst.rex_b());
        code.rex = Some(rex);

        // set opcode
        code.opcode = FlexBytes::from([0x89]);

        // set ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(dst.mode());
        mod_rm.set_reg(src.reg());
        mod_rm.set_rm(dst.rm());
        code.mod_rm = Some(mod_rm);

        code
    }
}

impl Mov<Reg64, Reg64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // set REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_r(src.rex_r());
        rex.set_x(false);
        rex.set_b(dst.rex_b());
        code.rex = Some(rex);

        // set opcode
        code.opcode = FlexBytes::from([0x89]);

        // set ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(dst.mode());
        mod_rm.set_reg(src.reg());
        mod_rm.set_rm(dst.rm());
        code.mod_rm = Some(mod_rm);

        code
    }
}

impl Mov<Reg64, u64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // set REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_b(dst.rex_b());
        code.rex = Some(rex);

        // set opcode
        code.opcode = FlexBytes::from([0xB8 + dst.reg()]);

        let mut imm = FlexBytes::new(8);
        imm.bytes_mut().write_u64::<LE>(src).unwrap();
        code.imm = imm;

        code
    }
}
