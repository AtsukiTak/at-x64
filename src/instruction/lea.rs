use crate::{
    mem::{Disp, Mem64},
    ByteCode, FlexBytes, ModRM, Reg64, Rex,
};
use byteorder::{WriteBytesExt as _, LE};

pub struct Lea<Dst, Src>(Dst, Src);

impl Lea<Reg64, Mem64> {
    pub fn new(dst: Reg64, src: Mem64) -> Self {
        Lea(dst, src)
    }

    pub fn bytecode(&self) -> ByteCode {
        let (dst, src) = (self.0, self.1);

        let mut code = ByteCode::new();

        // REX prefix
        let mut rex = Rex::new();
        rex.set_w(true);
        rex.set_r(dst.rex_r());
        rex.set_x(src.rex_x());
        rex.set_b(src.rex_b());
        code.rex = Some(rex);

        // opcode
        code.opcode = FlexBytes::from([0x8D]);

        // ModR/M
        let mut mod_rm = ModRM::new();
        mod_rm.set_mode(src.mode());
        mod_rm.set_reg(dst.reg());
        mod_rm.set_rm(src.rm());
        code.mod_rm = Some(mod_rm);

        // addr disp
        code.addr_disp = match src.disp() {
            None => FlexBytes::new(0),
            Some(Disp::U8(disp)) => FlexBytes::from([disp]),
            Some(Disp::U32(disp)) => {
                let mut bytes = FlexBytes::new(4);
                bytes.bytes_mut().write_u32::<LE>(disp).unwrap();
                bytes
            }
        };

        code
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cases = [
            (
                Lea::new(Reg64::RAX, Mem64::RegOffset(Reg64::RDI, 0)),
                vec![0x48, 0x8D, 0x07],
            ),
            (
                Lea::new(Reg64::RAX, Mem64::RegOffset(Reg64::RDI, 42)),
                vec![0x48, 0x8D, 0x47, 0x2A],
            ),
        ];

        for (origin, expected) in cases {
            assert_eq!(origin.bytecode().to_bytes().bytes(), expected);
        }
    }
}
