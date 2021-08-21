use crate::{ByteCode, BytesAtMost};

pub struct Syscall();

impl Syscall {
    pub fn bytecode(&self) -> ByteCode {
        let mut code = ByteCode::new();

        code.opcode = BytesAtMost::from([0x0f, 0x05]);

        code
    }
}
