#![allow(non_snake_case)]
#![allow(unused_variables)]

use z80::memory::MMU;
use z80::register::{ Register, RegisterList, Reg16 };
use z80::clock::Clock;
use z80::cpu::Flags;

// FIXME check the rotation instructions
// FIXME check the 16bit math op to make sure of the high/low bit

// 00
/// Do nothing
pub fn noop(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.set_clock(Clock::tick(1))
}

/// Load 16 bit immediate into BC
pub fn LDBCnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu
        .set(Register::C, mmu.read_byte(cpu.pc, &cpu))
        .set(Register::B, mmu.read_byte(cpu.pc+1, &cpu))
        .incr_pc(2)
        .set_clock(Clock::tick(3))
}

/// Write the value in register A in the memory at the address pointed to by BC
pub fn LDBCmA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let b = cpu.get(Register::B) as u16;
    let c = cpu.get(Register::C) as u16;
    let bc = (b << 8 ) + c;
    mmu.write_byte(bc.into(), cpu.get(Register::A));
    cpu.set_clock(Clock::tick(2))
}

/// 16 bit increment of BC
pub fn INCBC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu
        .incr_word(Register::C, Register::B)
        .set_clock(Clock::tick(1))
}

/// 8bit increment of b
pub fn INCr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.incr(Register::B).set_clock(Clock::tick(1))
}
/// 8bit decrement of b
pub fn DECr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.decr(Register::B).set_clock(Clock::tick(1))
}
/// Load 8bit in b
pub fn LDrn_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu
        .set(Register::B, mmu.read_byte(cpu.pc, &cpu))
        .incr_pc(1)
        .set_clock(Clock::tick(2))
}
/// Rotate akku left
pub fn RLCA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let bit7 = cpu.get(Register::A) & 0b1000000 != 0 ;
    let new_a = cpu.get(Register::A).rotate_left(1);
    cpu.set(Register::A, new_a)
        .reset_flag(Flags::ZERO)
        .reset_flag(Flags::OPERATION)
        .reset_flag(Flags::HALFCARRY)
        .set_flag(Flags::CARRY, bit7)
        .set_clock(Clock::tick(1))
}

/// Write stack pointer (SP) at the address in memory at PC
pub fn LDmmSP(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let addr = mmu.read_word(cpu.pc, &cpu);
    mmu.write_word(addr, cpu.sp);
    cpu.incr_pc(2).set_clock(Clock::tick(3))
}
/// Add HL and BC and store the result in HL
pub fn ADDHLBC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.add16(Register::H, Register::L, Register::B, Register::C).set_clock(Clock::tick(2))
}
/// Load to A the content of the memory at the address stored in BC
pub fn LDABCm(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let addr = ((cpu.get(Register::B) as Reg16) << 8) + cpu.get(Register::C) as Reg16;
    cpu
        .set(Register::A, mmu.read_byte(addr.into(), &cpu))
        .set_clock(Clock::tick(2))
}
/// 16bit decrement of BC
pub fn DECBC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.decr_word(Register::B, Register::C).set_clock(Clock::tick(2))
}

/// 8bit increment of C
pub fn INCr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.incr(Register::C).set_clock(Clock::tick(1))
}
/// 8bit decrement of C
pub fn DECr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.decr(Register::C).set_clock(Clock::tick(1))
}
/// Load the memory at the address in PC into C
pub fn LDrn_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.set(Register::C, mmu.read_byte(cpu.pc, &cpu))
        .incr_pc(1)
        .set_clock(Clock::tick(2))
}
/// Rotate akku right
pub fn RRCA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let bit0 = cpu.get(Register::A) & 0b1 == 1;
    let new_a = cpu.get(Register::A).rotate_right(1);
    cpu.set(Register::A, new_a)
        .reset_flag(Flags::ZERO)
        .reset_flag(Flags::OPERATION)
        .reset_flag(Flags::HALFCARRY)
        .set_flag(Flags::CARRY, bit0)
        .set_clock(Clock::tick(1))
}

// 10
/// On z80 it's DJNZn, on gameboy CPU it's STOP
/// STOP halt CPU and LCD until a button is pressed
pub fn STOP(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    // FIXME do something to wait until input???
    cpu.set_clock(Clock::tick(1))
}
/// Read 2 bytes from memory (at pc and pc+1) into DE
pub fn LDDEnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let e = mmu.read_byte(cpu.pc, &cpu);
    let d = mmu.read_byte(cpu.pc+1, &cpu);
    cpu.set(Register::E, e).set(Register::D, d).set_clock(Clock::tick(3)).incr_pc(2)
}
/// Write A at the address in DE
pub fn LDDEmA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let addr = ((cpu.get(Register::D) as Reg16) << 8) + cpu.get(Register::A) as Reg16;
    mmu.write_byte(addr, cpu.get(Register::A));
    cpu.set_clock(Clock::tick(2))
}
/// 16 bit incr of DE
pub fn INCDE(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.incr_word(Register::E, Register::D).set_clock(Clock::tick(2))
}

/// Increment D
pub fn INCr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.incr(Register::D).set_clock(Clock::tick(1))
}
/// Decrement D
pub fn DECr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.decr(Register::D).set_clock(Clock::tick(1))
}
/// Load 8bit at address PC into D
pub fn LDrn_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.set(Register::D, mmu.read_byte(cpu.pc, &cpu)).incr_pc(1).set_clock(Clock::tick(2))
}
/// Rotate A left using the carry
/// |7|6|5|4|3|2|1|0| + |Fc| => |6|5|4|3|2|1|0|Fc| + |7| (Fc = carry flag)
pub fn RLA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let val = cpu.get(Register::A);
    let carry = val & (1 << 7);
    let fc = if cpu.get_flag(Flags::CARRY) { 1 } else { 0 };
    cpu.set(Register::A, val.rotate_left(1) & fc)
        .set_flag(Flags::CARRY, carry != 0)
        .set_clock(Clock::tick(1))
}

pub fn JRn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JRn not implemented") }
pub fn ADDHLDE(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDHLDE not implemented") }
pub fn LDADEm(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDADEm not implemented") }
pub fn DECDE(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECDE not implemented") }

pub fn INCr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("INCr_e not implemented") }
pub fn DECr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECr_e not implemented") }
pub fn LDrn_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrn_e not implemented") }
pub fn RRA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RRA not implemented") }

// 20
pub fn JRNZn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JRNZn not implemented") }

/// Load 16bit imediate into HL
pub fn LDHLnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu
        .set(Register::L, mmu.read_byte(cpu.pc, &cpu))
        .set(Register::H, mmu.read_byte(cpu.pc+1, &cpu))
        .incr_pc(2)
        .set_clock(Clock::tick(3))
}
pub fn LDHLIA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLIA not implemented") }
pub fn INCHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("INCHL not implemented") }

pub fn INCr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("INCr_h not implemented") }
pub fn DECr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECr_h not implemented") }
pub fn LDrn_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrn_h not implemented") }
pub fn DAA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DAA not implemented") }

pub fn JRZn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JRZn not implemented") }
pub fn ADDHLHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDHLHL not implemented") }
pub fn LDAHLI(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDAHLI not implemented") }
pub fn DECHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECHL not implemented") }

pub fn INCr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("INCr_l not implemented") }
pub fn DECr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECr_l not implemented") }
pub fn LDrn_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrn_l not implemented") }
pub fn CPL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPL not implemented") }

// 30
pub fn JRNCn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JRNCn not implemented") }


/// Load 16 bit imediate in SP
pub fn LDSPnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.load_sp(mmu).incr_pc(2).set_clock(Clock::tick(3))
}
/// Save register A the address in the memory at the address saved
/// in HL and decrement HL
pub fn LDHLDA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let l = cpu.get(Register::L) as u16;
    let h = cpu.get(Register::H) as u16;
    let addr = l + (h << 8);
    mmu.write_byte(addr, cpu.get( Register::A ));
    cpu.decr_word(Register::L, Register::H).set_clock(Clock::tick(2))
}
pub fn INCSP(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("INCSP not implemented") }

pub fn INCHLm(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("INCHLm not implemented") }
pub fn DECHLm(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECHLm not implemented") }
pub fn LDHLmn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmn not implemented") }
pub fn SCF(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SCF not implemented") }

pub fn JRCn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JRCn not implemented") }
pub fn ADDHLSP(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDHLSP not implemented") }
pub fn LDAHLD(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDAHLD not implemented") }
pub fn DECSP(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECSP not implemented") }

pub fn INCr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("INCr_a not implemented") }
pub fn DECr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DECr_a not implemented") }
pub fn LDrn_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrn_a not implemented") }
pub fn CCF(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CCF not implemented") }

// 40
pub fn LDrr_bb(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_bb not implemented") }
pub fn LDrr_bc(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_bc not implemented") }
pub fn LDrr_bd(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_bd not implemented") }
pub fn LDrr_be(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_be not implemented") }

pub fn LDrr_bh(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_bh not implemented") }
pub fn LDrr_bl(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_bl not implemented") }
pub fn LDrHLm_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrHLm_b not implemented") }
pub fn LDrr_ba(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ba not implemented") }

pub fn LDrr_cb(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_cb not implemented") }
pub fn LDrr_cc(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_cc not implemented") }
pub fn LDrr_cd(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_cd not implemented") }
pub fn LDrr_ce(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ce not implemented") }

pub fn LDrr_ch(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ch not implemented") }
pub fn LDrr_cl(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_cl not implemented") }
pub fn LDrHLm_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrHLm_c not implemented") }
pub fn LDrr_ca(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ca not implemented") }

// 50
pub fn LDrr_db(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_db not implemented") }
pub fn LDrr_dc(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_dc not implemented") }
pub fn LDrr_dd(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_dd not implemented") }
pub fn LDrr_de(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_de not implemented") }

pub fn LDrr_dh(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_dh not implemented") }
pub fn LDrr_dl(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_dl not implemented") }
pub fn LDrHLm_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrHLm_d not implemented") }
pub fn LDrr_da(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_da not implemented") }

pub fn LDrr_eb(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_eb not implemented") }
pub fn LDrr_ec(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ec not implemented") }
pub fn LDrr_ed(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ed not implemented") }
pub fn LDrr_ee(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ee not implemented") }

pub fn LDrr_eh(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_eh not implemented") }
pub fn LDrr_el(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_el not implemented") }
pub fn LDrHLm_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrHLm_e not implemented") }
pub fn LDrr_ea(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ea not implemented") }

// 60
pub fn LDrr_hb(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_hb not implemented") }
pub fn LDrr_hc(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_hc not implemented") }
pub fn LDrr_hd(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_hd not implemented") }
pub fn LDrr_he(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_he not implemented") }

pub fn LDrr_hh(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_hh not implemented") }
pub fn LDrr_hl(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_hl not implemented") }
pub fn LDrHLm_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrHLm_h not implemented") }
pub fn LDrr_ha(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ha not implemented") }

pub fn LDrr_lb(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_lb not implemented") }
pub fn LDrr_lc(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_lc not implemented") }
pub fn LDrr_ld(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ld not implemented") }
pub fn LDrr_le(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_le not implemented") }

pub fn LDrr_lh(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_lh not implemented") }
pub fn LDrr_ll(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ll not implemented") }
pub fn LDrHLm_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrHLm_l not implemented") }
pub fn LDrr_la(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_la not implemented") }

// 70
pub fn LDHLmr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmr_b not implemented") }
pub fn LDHLmr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmr_c not implemented") }
pub fn LDHLmr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmr_d not implemented") }
pub fn LDHLmr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmr_e not implemented") }

pub fn LDHLmr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmr_h not implemented") }
pub fn LDHLmr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmr_l not implemented") }
pub fn HALT(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("HALT not implemented") }
pub fn LDHLmr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLmr_a not implemented") }

pub fn LDrr_ab(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ab not implemented") }
pub fn LDrr_ac(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ac not implemented") }
pub fn LDrr_ad(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ad not implemented") }
pub fn LDrr_ae(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ae not implemented") }

pub fn LDrr_ah(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_ah not implemented") }
pub fn LDrr_al(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_al not implemented") }
pub fn LDrHLm_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrHLm_a not implemented") }
pub fn LDrr_aa(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDrr_aa not implemented") }

// 80
pub fn ADDr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDr_b not implemented") }
pub fn ADDr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDr_c not implemented") }
pub fn ADDr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDr_d not implemented") }

/// Add register E to register A and save in register A
pub fn ADDr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.add(Register::A, cpu.get(Register::E)).set_clock(Clock::tick(1))
}

pub fn ADDr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDr_h not implemented") }
pub fn ADDr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDr_l not implemented") }
pub fn ADDHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDHL not implemented") }
pub fn ADDr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDr_a not implemented") }

pub fn ADCr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCr_b not implemented") }
pub fn ADCr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCr_c not implemented") }
pub fn ADCr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCr_d not implemented") }
pub fn ADCr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCr_e not implemented") }

pub fn ADCr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCr_h not implemented") }
pub fn ADCr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCr_l not implemented") }
pub fn ADCHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCHL not implemented") }
pub fn ADCr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCr_a not implemented") }

// 90
pub fn SUBr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBr_b not implemented") }
pub fn SUBr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBr_c not implemented") }
pub fn SUBr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBr_d not implemented") }
pub fn SUBr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBr_e not implemented") }

pub fn SUBr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBr_h not implemented") }
pub fn SUBr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBr_l not implemented") }
pub fn SUBHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBHL not implemented") }
pub fn SUBr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBr_a not implemented") }

pub fn SBCr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCr_b not implemented") }
pub fn SBCr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCr_c not implemented") }
pub fn SBCr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCr_d not implemented") }
pub fn SBCr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCr_e not implemented") }

pub fn SBCr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCr_h not implemented") }
pub fn SBCr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCr_l not implemented") }
pub fn SBCHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCHL not implemented") }
pub fn SBCr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCr_a not implemented") }

// A0
pub fn ANDr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDr_b not implemented") }
pub fn ANDr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDr_c not implemented") }
pub fn ANDr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDr_d not implemented") }
pub fn ANDr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDr_e not implemented") }

pub fn ANDr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDr_h not implemented") }
pub fn ANDr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDr_l not implemented") }
pub fn ANDHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDHL not implemented") }
pub fn ANDr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDr_a not implemented") }

pub fn XORr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORr_b not implemented") }
pub fn XORr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORr_c not implemented") }
pub fn XORr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORr_d not implemented") }
pub fn XORr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORr_e not implemented") }

pub fn XORr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORr_h not implemented") }
pub fn XORr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORr_l not implemented") }
pub fn XORHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORHL not implemented") }

/// XOR register a with itself
pub fn XORr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.xor(Register::A, Register::A).set_clock(Clock::tick(1))
}

// B0
pub fn ORr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORr_b not implemented") }
pub fn ORr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORr_c not implemented") }
pub fn ORr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORr_d not implemented") }
pub fn ORr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORr_e not implemented") }

pub fn ORr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORr_h not implemented") }
pub fn ORr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORr_l not implemented") }
pub fn ORHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORHL not implemented") }
pub fn ORr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORr_a not implemented") }

/// Compare register A to register B, set the flag according to the comparison
pub fn CPr_b(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    // The result of the comparison is stored in the flag register.
    // The result is computed by substracting A and B and setting the flags according to the
    // substraction
    // Underflow -> B > A
    // Zero      -> B = A
    // Nothing   -> B < A
    cpu.cmp(Register::A, Register::B).set_clock(Clock::tick(1))
}
pub fn CPr_c(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPr_c not implemented") }
pub fn CPr_d(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPr_d not implemented") }
pub fn CPr_e(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPr_e not implemented") }

pub fn CPr_h(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPr_h not implemented") }
pub fn CPr_l(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPr_l not implemented") }
pub fn CPHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPHL not implemented") }
pub fn CPr_a(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPr_a not implemented") }

// C0
pub fn RETNZ(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RETNZ not implemented") }
pub fn POPBC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("POPBC not implemented") }
pub fn JPNZnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JPNZnn not implemented") }
pub fn JPnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JPnn not implemented") }

pub fn CALLNZnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CALLNZnn not implemented") }

/// Push registers B and C to the stack
pub fn PUSHBC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu
        .push_stach(mmu, cpu.get( Register::B ))
        .push_stach(mmu, cpu.get( Register::C ))
        .set_clock(Clock::tick(3))
}
pub fn ADDn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDn not implemented") }
pub fn RST00(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST00 not implemented") }

pub fn RETZ(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RETZ not implemented") }
pub fn RET(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RET not implemented") }
pub fn JPZnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JPZnn not implemented") }
pub fn MAPcb(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("MAPcb not implemented") }

pub fn CALLZnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CALLZnn not implemented") }
pub fn CALLnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CALLnn not implemented") }
pub fn ADCn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADCn not implemented") }
pub fn RST08(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST08 not implemented") }

// D0
pub fn RETNC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RETNC not implemented") }
pub fn POPDE(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("POPDE not implemented") }
pub fn JPNCnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JPNCnn not implemented") }
pub fn XX(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XX not implemented") }

pub fn CALLNCnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CALLNCnn not implemented") }
pub fn PUSHDE(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("PUSHDE not implemented") }
pub fn SUBn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SUBn not implemented") }
pub fn RST10(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST10 not implemented") }

pub fn RETC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RETC not implemented") }
pub fn RETI(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RETI not implemented") }
pub fn JPCnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JPCnn not implemented") }

pub fn CALLCnn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CALLCnn not implemented") }
pub fn SBCn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("SBCn not implemented") }
pub fn RST18(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST18 not implemented") }

// E0
pub fn LDIOnA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDIOnA not implemented") }

/// Pop register H & L off the stack
pub fn POPHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    cpu.pop_stack(mmu, Register::L)
        .pop_stack(mmu, Register::H)
        .set_clock(Clock::tick(3))
}
pub fn LDIOCA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDIOCA not implemented") }

pub fn PUSHHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("PUSHHL not implemented") }
pub fn ANDn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ANDn not implemented") }
pub fn RST20(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST20 not implemented") }

pub fn ADDSPn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ADDSPn not implemented") }
pub fn JPHL(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("JPHL not implemented") }
pub fn LDmmA(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDmmA not implemented") }

pub fn XORn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("XORn not implemented") }
pub fn RST28(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST28 not implemented") }

// F0
pub fn LDAIOn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDAIOn not implemented") }
pub fn POPAF(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("POPAF not implemented") }
pub fn LDAIOC(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDAIOC not implemented") }
pub fn DI(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("DI not implemented") }

pub fn PUSHAF(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("PUSHAF not implemented") }
pub fn ORn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("ORn not implemented") }
pub fn RST30(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST30 not implemented") }

pub fn LDHLSPn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("LDHLSPn not implemented") }
pub fn LDAmm(cpu: RegisterList, mmu: &mut MMU) -> RegisterList {
    let addr = mmu.read_word(cpu.pc, &cpu);
    cpu.incr_pc(2).set(Register::A, mmu.read_byte(addr, &cpu)).set_clock(Clock::tick(3))
}
pub fn EI(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("EI not implemented") }

pub fn CPn(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("CPn not implemented") }
pub fn RST38(cpu: RegisterList, mmu: &mut MMU) -> RegisterList { unimplemented!("RST38 not implemented") }
