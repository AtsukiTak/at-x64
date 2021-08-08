use crate::{ByteCode, FlexBytes, Mem64, ModRM, Reg64, Rex};

pub struct Lea<Dst, Src>(Dst, Src);

impl Lea<Reg64, Mem64> {
    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // set REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_r(dst.rex_r());
        rex.set_x(src.rex_x());
        rex.set_b(src.rex_b());
        code.rex = Some(rex);

        // set opcode
        code.opcode = FlexBytes::from([0x8D]);

        // set ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(src.mode());
        mod_rm.set_reg(dst.reg());
        mod_rm.set_rm(src.rm());
        code.mod_rm = Some(mod_rm);

        code
    }
}
