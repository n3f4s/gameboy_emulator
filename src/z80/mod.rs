
// TODO : allow only some public function
// FIXME : put opcodes in their own mod
#[macro_use]
pub mod cpu;
pub mod memory;
pub mod opcodes;
pub mod opcode_map;
pub mod register;
pub mod clock;
pub mod instructions;
pub mod gpu;
