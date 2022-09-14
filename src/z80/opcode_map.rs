
use crate::z80::opcodes::*;
use crate::z80::register::RegisterList;
use crate::z80::memory::MMU;

impl<'a> OpcodeMap<'a> {
    pub fn map(&self, opcode: Opcode, cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
        println!("Executing opcode {} (0x{:x})", self.op_name[opcode], opcode);
        self.op_map[opcode](cpu, mmu)
    }
}
