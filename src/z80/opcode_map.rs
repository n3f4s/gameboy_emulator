
use z80::opcodes::*;
use z80::cpu::CPU;

impl<'a> OpcodeMap<'a> {
    pub fn map(&self, opcode: Opcode, cpu: &mut CPU) {
        println!("Executing opcode {} (0x{:x})", self.op_name[opcode], opcode);
        self.op_map[opcode](cpu)
    }
}
