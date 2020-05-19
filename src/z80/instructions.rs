#![allow(non_snake_case)]
#![allow(unused_variables)]

use z80::cpu::{ CPU, Flags };

// 00
/// Do nothing
pub fn noop(cpu: &mut CPU) {
    cpu.clock.tick(1);
}

/// Load 16 bit immediate into BC
pub fn LDBCnn(cpu: &mut CPU) {
    cpu.registers.c = cpu.mmu.read_byte(cpu.registers.pc, &cpu.registers);
    cpu.registers.b = cpu.mmu.read_byte(cpu.registers.pc + 1, &cpu.registers);
    cpu.registers.pc += 2;
    cpu.registers.clock.m = 3; // FIXME ????
}

/// Write the value in register A in the memory at the address pointed to by BC
pub fn LDBCmA(cpu: &mut CPU) {
    let b = cpu.registers.b as u16;
    let c = cpu.registers.c as u16;
    let bc = (b << 8 ) + c;
    cpu.mmu.write_byte(bc.into(), cpu.registers.a);
    cpu.registers.clock.m = 2;
}

// FIXME : use wrapping?
/// 16 bit increment of BC
pub fn INCBC(cpu: &mut CPU) {
    cpu.registers.c = (cpu.registers.c + 1) & 255;
    if cpu.registers.c == 0 {
        cpu.registers.b = (cpu.registers.b + 1) & 255
    };
    cpu.registers.clock.m=1;
}

pub fn INCr_b(cpu: &mut CPU) { unimplemented!("INCr_b not implemented") }
pub fn DECr_b(cpu: &mut CPU) { unimplemented!("DECr_b not implemented") }
pub fn LDrn_b(cpu: &mut CPU) { unimplemented!("LDrn_b not implemented") }
pub fn RLCA(cpu: &mut CPU) { unimplemented!("RLCA not implemented") }

pub fn LDmmSP(cpu: &mut CPU) { unimplemented!("LDmmSP not implemented") }
pub fn ADDHLBC(cpu: &mut CPU) { unimplemented!("ADDHLBC not implemented") }
pub fn LDABCm(cpu: &mut CPU) { unimplemented!("LDABCm not implemented") }
pub fn DECBC(cpu: &mut CPU) { unimplemented!("DECBC not implemented") }

pub fn INCr_c(cpu: &mut CPU) { unimplemented!("INCr_c not implemented") }
pub fn DECr_c(cpu: &mut CPU) { unimplemented!("DECr_c not implemented") }
pub fn LDrn_c(cpu: &mut CPU) { unimplemented!("LDrn_c not implemented") }
pub fn RRCA(cpu: &mut CPU) { unimplemented!("RRCA not implemented") }

// 10
pub fn DJNZn(cpu: &mut CPU) { unimplemented!("DJNZn not implemented") }
pub fn LDDEnn(cpu: &mut CPU) { unimplemented!("LDDEnn not implemented") }
pub fn LDDEmA(cpu: &mut CPU) { unimplemented!("LDDEmA not implemented") }
pub fn INCDE(cpu: &mut CPU) { unimplemented!("INCDE not implemented") }

pub fn INCr_d(cpu: &mut CPU) { unimplemented!("INCr_d not implemented") }
pub fn DECr_d(cpu: &mut CPU) { unimplemented!("DECr_d not implemented") }
pub fn LDrn_d(cpu: &mut CPU) { unimplemented!("LDrn_d not implemented") }
pub fn RLA(cpu: &mut CPU) { unimplemented!("RLA not implemented") }

pub fn JRn(cpu: &mut CPU) { unimplemented!("JRn not implemented") }
pub fn ADDHLDE(cpu: &mut CPU) { unimplemented!("ADDHLDE not implemented") }
pub fn LDADEm(cpu: &mut CPU) { unimplemented!("LDADEm not implemented") }
pub fn DECDE(cpu: &mut CPU) { unimplemented!("DECDE not implemented") }

pub fn INCr_e(cpu: &mut CPU) { unimplemented!("INCr_e not implemented") }
pub fn DECr_e(cpu: &mut CPU) { unimplemented!("DECr_e not implemented") }
pub fn LDrn_e(cpu: &mut CPU) { unimplemented!("LDrn_e not implemented") }
pub fn RRA(cpu: &mut CPU) { unimplemented!("RRA not implemented") }

// 20
pub fn JRNZn(cpu: &mut CPU) { unimplemented!("JRNZn not implemented") }
pub fn LDHLnn(cpu: &mut CPU) { unimplemented!("LDHLnn not implemented") }
pub fn LDHLIA(cpu: &mut CPU) { unimplemented!("LDHLIA not implemented") }
pub fn INCHL(cpu: &mut CPU) { unimplemented!("INCHL not implemented") }

pub fn INCr_h(cpu: &mut CPU) { unimplemented!("INCr_h not implemented") }
pub fn DECr_h(cpu: &mut CPU) { unimplemented!("DECr_h not implemented") }
pub fn LDrn_h(cpu: &mut CPU) { unimplemented!("LDrn_h not implemented") }
pub fn DAA(cpu: &mut CPU) { unimplemented!("DAA not implemented") }

pub fn JRZn(cpu: &mut CPU) { unimplemented!("JRZn not implemented") }
pub fn ADDHLHL(cpu: &mut CPU) { unimplemented!("ADDHLHL not implemented") }
pub fn LDAHLI(cpu: &mut CPU) { unimplemented!("LDAHLI not implemented") }
pub fn DECHL(cpu: &mut CPU) { unimplemented!("DECHL not implemented") }

pub fn INCr_l(cpu: &mut CPU) { unimplemented!("INCr_l not implemented") }
pub fn DECr_l(cpu: &mut CPU) { unimplemented!("DECr_l not implemented") }
pub fn LDrn_l(cpu: &mut CPU) { unimplemented!("LDrn_l not implemented") }
pub fn CPL(cpu: &mut CPU) { unimplemented!("CPL not implemented") }

// 30
pub fn JRNCn(cpu: &mut CPU) { unimplemented!("JRNCn not implemented") }


/// Load 16 bit imediate in SP
pub fn LDSPnn(cpu: &mut CPU) {
    cpu.registers.sp = cpu.mmu.read_word(cpu.registers.pc, &cpu.registers);
    cpu.registers.pc += 2;
    cpu.registers.clock.m = 3;
}
pub fn LDHLDA(cpu: &mut CPU) { unimplemented!("LDHLDA not implemented") }
pub fn INCSP(cpu: &mut CPU) { unimplemented!("INCSP not implemented") }

pub fn INCHLm(cpu: &mut CPU) { unimplemented!("INCHLm not implemented") }
pub fn DECHLm(cpu: &mut CPU) { unimplemented!("DECHLm not implemented") }
pub fn LDHLmn(cpu: &mut CPU) { unimplemented!("LDHLmn not implemented") }
pub fn SCF(cpu: &mut CPU) { unimplemented!("SCF not implemented") }

pub fn JRCn(cpu: &mut CPU) { unimplemented!("JRCn not implemented") }
pub fn ADDHLSP(cpu: &mut CPU) { unimplemented!("ADDHLSP not implemented") }
pub fn LDAHLD(cpu: &mut CPU) { unimplemented!("LDAHLD not implemented") }
pub fn DECSP(cpu: &mut CPU) { unimplemented!("DECSP not implemented") }

pub fn INCr_a(cpu: &mut CPU) { unimplemented!("INCr_a not implemented") }
pub fn DECr_a(cpu: &mut CPU) { unimplemented!("DECr_a not implemented") }
pub fn LDrn_a(cpu: &mut CPU) { unimplemented!("LDrn_a not implemented") }
pub fn CCF(cpu: &mut CPU) { unimplemented!("CCF not implemented") }

// 40
pub fn LDrr_bb(cpu: &mut CPU) { unimplemented!("LDrr_bb not implemented") }
pub fn LDrr_bc(cpu: &mut CPU) { unimplemented!("LDrr_bc not implemented") }
pub fn LDrr_bd(cpu: &mut CPU) { unimplemented!("LDrr_bd not implemented") }
pub fn LDrr_be(cpu: &mut CPU) { unimplemented!("LDrr_be not implemented") }

pub fn LDrr_bh(cpu: &mut CPU) { unimplemented!("LDrr_bh not implemented") }
pub fn LDrr_bl(cpu: &mut CPU) { unimplemented!("LDrr_bl not implemented") }
pub fn LDrHLm_b(cpu: &mut CPU) { unimplemented!("LDrHLm_b not implemented") }
pub fn LDrr_ba(cpu: &mut CPU) { unimplemented!("LDrr_ba not implemented") }

pub fn LDrr_cb(cpu: &mut CPU) { unimplemented!("LDrr_cb not implemented") }
pub fn LDrr_cc(cpu: &mut CPU) { unimplemented!("LDrr_cc not implemented") }
pub fn LDrr_cd(cpu: &mut CPU) { unimplemented!("LDrr_cd not implemented") }
pub fn LDrr_ce(cpu: &mut CPU) { unimplemented!("LDrr_ce not implemented") }

pub fn LDrr_ch(cpu: &mut CPU) { unimplemented!("LDrr_ch not implemented") }
pub fn LDrr_cl(cpu: &mut CPU) { unimplemented!("LDrr_cl not implemented") }
pub fn LDrHLm_c(cpu: &mut CPU) { unimplemented!("LDrHLm_c not implemented") }
pub fn LDrr_ca(cpu: &mut CPU) { unimplemented!("LDrr_ca not implemented") }

// 50
pub fn LDrr_db(cpu: &mut CPU) { unimplemented!("LDrr_db not implemented") }
pub fn LDrr_dc(cpu: &mut CPU) { unimplemented!("LDrr_dc not implemented") }
pub fn LDrr_dd(cpu: &mut CPU) { unimplemented!("LDrr_dd not implemented") }
pub fn LDrr_de(cpu: &mut CPU) { unimplemented!("LDrr_de not implemented") }

pub fn LDrr_dh(cpu: &mut CPU) { unimplemented!("LDrr_dh not implemented") }
pub fn LDrr_dl(cpu: &mut CPU) { unimplemented!("LDrr_dl not implemented") }
pub fn LDrHLm_d(cpu: &mut CPU) { unimplemented!("LDrHLm_d not implemented") }
pub fn LDrr_da(cpu: &mut CPU) { unimplemented!("LDrr_da not implemented") }

pub fn LDrr_eb(cpu: &mut CPU) { unimplemented!("LDrr_eb not implemented") }
pub fn LDrr_ec(cpu: &mut CPU) { unimplemented!("LDrr_ec not implemented") }
pub fn LDrr_ed(cpu: &mut CPU) { unimplemented!("LDrr_ed not implemented") }
pub fn LDrr_ee(cpu: &mut CPU) { unimplemented!("LDrr_ee not implemented") }

pub fn LDrr_eh(cpu: &mut CPU) { unimplemented!("LDrr_eh not implemented") }
pub fn LDrr_el(cpu: &mut CPU) { unimplemented!("LDrr_el not implemented") }
pub fn LDrHLm_e(cpu: &mut CPU) { unimplemented!("LDrHLm_e not implemented") }
pub fn LDrr_ea(cpu: &mut CPU) { unimplemented!("LDrr_ea not implemented") }

// 60
pub fn LDrr_hb(cpu: &mut CPU) { unimplemented!("LDrr_hb not implemented") }
pub fn LDrr_hc(cpu: &mut CPU) { unimplemented!("LDrr_hc not implemented") }
pub fn LDrr_hd(cpu: &mut CPU) { unimplemented!("LDrr_hd not implemented") }
pub fn LDrr_he(cpu: &mut CPU) { unimplemented!("LDrr_he not implemented") }

pub fn LDrr_hh(cpu: &mut CPU) { unimplemented!("LDrr_hh not implemented") }
pub fn LDrr_hl(cpu: &mut CPU) { unimplemented!("LDrr_hl not implemented") }
pub fn LDrHLm_h(cpu: &mut CPU) { unimplemented!("LDrHLm_h not implemented") }
pub fn LDrr_ha(cpu: &mut CPU) { unimplemented!("LDrr_ha not implemented") }

pub fn LDrr_lb(cpu: &mut CPU) { unimplemented!("LDrr_lb not implemented") }
pub fn LDrr_lc(cpu: &mut CPU) { unimplemented!("LDrr_lc not implemented") }
pub fn LDrr_ld(cpu: &mut CPU) { unimplemented!("LDrr_ld not implemented") }
pub fn LDrr_le(cpu: &mut CPU) { unimplemented!("LDrr_le not implemented") }

pub fn LDrr_lh(cpu: &mut CPU) { unimplemented!("LDrr_lh not implemented") }
pub fn LDrr_ll(cpu: &mut CPU) { unimplemented!("LDrr_ll not implemented") }
pub fn LDrHLm_l(cpu: &mut CPU) { unimplemented!("LDrHLm_l not implemented") }
pub fn LDrr_la(cpu: &mut CPU) { unimplemented!("LDrr_la not implemented") }

// 70
pub fn LDHLmr_b(cpu: &mut CPU) { unimplemented!("LDHLmr_b not implemented") }
pub fn LDHLmr_c(cpu: &mut CPU) { unimplemented!("LDHLmr_c not implemented") }
pub fn LDHLmr_d(cpu: &mut CPU) { unimplemented!("LDHLmr_d not implemented") }
pub fn LDHLmr_e(cpu: &mut CPU) { unimplemented!("LDHLmr_e not implemented") }

pub fn LDHLmr_h(cpu: &mut CPU) { unimplemented!("LDHLmr_h not implemented") }
pub fn LDHLmr_l(cpu: &mut CPU) { unimplemented!("LDHLmr_l not implemented") }
pub fn HALT(cpu: &mut CPU) { unimplemented!("HALT not implemented") }
pub fn LDHLmr_a(cpu: &mut CPU) { unimplemented!("LDHLmr_a not implemented") }

pub fn LDrr_ab(cpu: &mut CPU) { unimplemented!("LDrr_ab not implemented") }
pub fn LDrr_ac(cpu: &mut CPU) { unimplemented!("LDrr_ac not implemented") }
pub fn LDrr_ad(cpu: &mut CPU) { unimplemented!("LDrr_ad not implemented") }
pub fn LDrr_ae(cpu: &mut CPU) { unimplemented!("LDrr_ae not implemented") }

pub fn LDrr_ah(cpu: &mut CPU) { unimplemented!("LDrr_ah not implemented") }
pub fn LDrr_al(cpu: &mut CPU) { unimplemented!("LDrr_al not implemented") }
pub fn LDrHLm_a(cpu: &mut CPU) { unimplemented!("LDrHLm_a not implemented") }
pub fn LDrr_aa(cpu: &mut CPU) { unimplemented!("LDrr_aa not implemented") }

// 80
pub fn ADDr_b(cpu: &mut CPU) { unimplemented!("ADDr_b not implemented") }
pub fn ADDr_c(cpu: &mut CPU) { unimplemented!("ADDr_c not implemented") }
pub fn ADDr_d(cpu: &mut CPU) { unimplemented!("ADDr_d not implemented") }
pub fn ADDr_e(cpu: &mut CPU) { unimplemented!("ADDr_e not implemented") }

pub fn ADDr_h(cpu: &mut CPU) { unimplemented!("ADDr_h not implemented") }
pub fn ADDr_l(cpu: &mut CPU) { unimplemented!("ADDr_l not implemented") }
pub fn ADDHL(cpu: &mut CPU) { unimplemented!("ADDHL not implemented") }
pub fn ADDr_a(cpu: &mut CPU) { unimplemented!("ADDr_a not implemented") }

pub fn ADCr_b(cpu: &mut CPU) { unimplemented!("ADCr_b not implemented") }
pub fn ADCr_c(cpu: &mut CPU) { unimplemented!("ADCr_c not implemented") }
pub fn ADCr_d(cpu: &mut CPU) { unimplemented!("ADCr_d not implemented") }
pub fn ADCr_e(cpu: &mut CPU) { unimplemented!("ADCr_e not implemented") }

pub fn ADCr_h(cpu: &mut CPU) { unimplemented!("ADCr_h not implemented") }
pub fn ADCr_l(cpu: &mut CPU) { unimplemented!("ADCr_l not implemented") }
pub fn ADCHL(cpu: &mut CPU) { unimplemented!("ADCHL not implemented") }
pub fn ADCr_a(cpu: &mut CPU) { unimplemented!("ADCr_a not implemented") }

// 90
pub fn SUBr_b(cpu: &mut CPU) { unimplemented!("SUBr_b not implemented") }
pub fn SUBr_c(cpu: &mut CPU) { unimplemented!("SUBr_c not implemented") }
pub fn SUBr_d(cpu: &mut CPU) { unimplemented!("SUBr_d not implemented") }
pub fn SUBr_e(cpu: &mut CPU) { unimplemented!("SUBr_e not implemented") }

pub fn SUBr_h(cpu: &mut CPU) { unimplemented!("SUBr_h not implemented") }
pub fn SUBr_l(cpu: &mut CPU) { unimplemented!("SUBr_l not implemented") }
pub fn SUBHL(cpu: &mut CPU) { unimplemented!("SUBHL not implemented") }
pub fn SUBr_a(cpu: &mut CPU) { unimplemented!("SUBr_a not implemented") }

pub fn SBCr_b(cpu: &mut CPU) { unimplemented!("SBCr_b not implemented") }
pub fn SBCr_c(cpu: &mut CPU) { unimplemented!("SBCr_c not implemented") }
pub fn SBCr_d(cpu: &mut CPU) { unimplemented!("SBCr_d not implemented") }
pub fn SBCr_e(cpu: &mut CPU) { unimplemented!("SBCr_e not implemented") }

pub fn SBCr_h(cpu: &mut CPU) { unimplemented!("SBCr_h not implemented") }
pub fn SBCr_l(cpu: &mut CPU) { unimplemented!("SBCr_l not implemented") }
pub fn SBCHL(cpu: &mut CPU) { unimplemented!("SBCHL not implemented") }
pub fn SBCr_a(cpu: &mut CPU) { unimplemented!("SBCr_a not implemented") }

// A0
pub fn ANDr_b(cpu: &mut CPU) { unimplemented!("ANDr_b not implemented") }
pub fn ANDr_c(cpu: &mut CPU) { unimplemented!("ANDr_c not implemented") }
pub fn ANDr_d(cpu: &mut CPU) { unimplemented!("ANDr_d not implemented") }
pub fn ANDr_e(cpu: &mut CPU) { unimplemented!("ANDr_e not implemented") }

pub fn ANDr_h(cpu: &mut CPU) { unimplemented!("ANDr_h not implemented") }
pub fn ANDr_l(cpu: &mut CPU) { unimplemented!("ANDr_l not implemented") }
pub fn ANDHL(cpu: &mut CPU) { unimplemented!("ANDHL not implemented") }
pub fn ANDr_a(cpu: &mut CPU) { unimplemented!("ANDr_a not implemented") }

pub fn XORr_b(cpu: &mut CPU) { unimplemented!("XORr_b not implemented") }
pub fn XORr_c(cpu: &mut CPU) { unimplemented!("XORr_c not implemented") }
pub fn XORr_d(cpu: &mut CPU) { unimplemented!("XORr_d not implemented") }
pub fn XORr_e(cpu: &mut CPU) { unimplemented!("XORr_e not implemented") }

pub fn XORr_h(cpu: &mut CPU) { unimplemented!("XORr_h not implemented") }
pub fn XORr_l(cpu: &mut CPU) { unimplemented!("XORr_l not implemented") }
pub fn XORHL(cpu: &mut CPU) { unimplemented!("XORHL not implemented") }

/// XOR register 1 with itself
pub fn XORr_a(cpu: &mut CPU) {
    cpu.registers.a ^= cpu.registers.a;
    cpu.registers.a &= 255;
    cpu.registers.f = if cpu.registers.a != 0 {
        0
    } else {
        Flags::ZERO as u8
    };
    cpu.registers.clock.m = 1;
}

// B0
pub fn ORr_b(cpu: &mut CPU) { unimplemented!("ORr_b not implemented") }
pub fn ORr_c(cpu: &mut CPU) { unimplemented!("ORr_c not implemented") }
pub fn ORr_d(cpu: &mut CPU) { unimplemented!("ORr_d not implemented") }
pub fn ORr_e(cpu: &mut CPU) { unimplemented!("ORr_e not implemented") }

pub fn ORr_h(cpu: &mut CPU) { unimplemented!("ORr_h not implemented") }
pub fn ORr_l(cpu: &mut CPU) { unimplemented!("ORr_l not implemented") }
pub fn ORHL(cpu: &mut CPU) { unimplemented!("ORHL not implemented") }
pub fn ORr_a(cpu: &mut CPU) { unimplemented!("ORr_a not implemented") }

pub fn CPr_b(cpu: &mut CPU) { unimplemented!("CPr_b not implemented") }
pub fn CPr_c(cpu: &mut CPU) { unimplemented!("CPr_c not implemented") }
pub fn CPr_d(cpu: &mut CPU) { unimplemented!("CPr_d not implemented") }
pub fn CPr_e(cpu: &mut CPU) { unimplemented!("CPr_e not implemented") }

pub fn CPr_h(cpu: &mut CPU) { unimplemented!("CPr_h not implemented") }
pub fn CPr_l(cpu: &mut CPU) { unimplemented!("CPr_l not implemented") }
pub fn CPHL(cpu: &mut CPU) { unimplemented!("CPHL not implemented") }
pub fn CPr_a(cpu: &mut CPU) { unimplemented!("CPr_a not implemented") }

// C0
pub fn RETNZ(cpu: &mut CPU) { unimplemented!("RETNZ not implemented") }
pub fn POPBC(cpu: &mut CPU) { unimplemented!("POPBC not implemented") }
pub fn JPNZnn(cpu: &mut CPU) { unimplemented!("JPNZnn not implemented") }
pub fn JPnn(cpu: &mut CPU) { unimplemented!("JPnn not implemented") }

pub fn CALLNZnn(cpu: &mut CPU) { unimplemented!("CALLNZnn not implemented") }
pub fn PUSHBC(cpu: &mut CPU) { unimplemented!("PUSHBC not implemented") }
pub fn ADDn(cpu: &mut CPU) { unimplemented!("ADDn not implemented") }
pub fn RST00(cpu: &mut CPU) { unimplemented!("RST00 not implemented") }

pub fn RETZ(cpu: &mut CPU) { unimplemented!("RETZ not implemented") }
pub fn RET(cpu: &mut CPU) { unimplemented!("RET not implemented") }
pub fn JPZnn(cpu: &mut CPU) { unimplemented!("JPZnn not implemented") }
pub fn MAPcb(cpu: &mut CPU) { unimplemented!("MAPcb not implemented") }

pub fn CALLZnn(cpu: &mut CPU) { unimplemented!("CALLZnn not implemented") }
pub fn CALLnn(cpu: &mut CPU) { unimplemented!("CALLnn not implemented") }
pub fn ADCn(cpu: &mut CPU) { unimplemented!("ADCn not implemented") }
pub fn RST08(cpu: &mut CPU) { unimplemented!("RST08 not implemented") }

// D0
pub fn RETNC(cpu: &mut CPU) { unimplemented!("RETNC not implemented") }
pub fn POPDE(cpu: &mut CPU) { unimplemented!("POPDE not implemented") }
pub fn JPNCnn(cpu: &mut CPU) { unimplemented!("JPNCnn not implemented") }
pub fn XX(cpu: &mut CPU) { unimplemented!("XX not implemented") }

pub fn CALLNCnn(cpu: &mut CPU) { unimplemented!("CALLNCnn not implemented") }
pub fn PUSHDE(cpu: &mut CPU) { unimplemented!("PUSHDE not implemented") }
pub fn SUBn(cpu: &mut CPU) { unimplemented!("SUBn not implemented") }
pub fn RST10(cpu: &mut CPU) { unimplemented!("RST10 not implemented") }

pub fn RETC(cpu: &mut CPU) { unimplemented!("RETC not implemented") }
pub fn RETI(cpu: &mut CPU) { unimplemented!("RETI not implemented") }
pub fn JPCnn(cpu: &mut CPU) { unimplemented!("JPCnn not implemented") }

pub fn CALLCnn(cpu: &mut CPU) { unimplemented!("CALLCnn not implemented") }
pub fn SBCn(cpu: &mut CPU) { unimplemented!("SBCn not implemented") }
pub fn RST18(cpu: &mut CPU) { unimplemented!("RST18 not implemented") }

// E0
pub fn LDIOnA(cpu: &mut CPU) { unimplemented!("LDIOnA not implemented") }
pub fn POPHL(cpu: &mut CPU) { unimplemented!("POPHL not implemented") }
pub fn LDIOCA(cpu: &mut CPU) { unimplemented!("LDIOCA not implemented") }

pub fn PUSHHL(cpu: &mut CPU) { unimplemented!("PUSHHL not implemented") }
pub fn ANDn(cpu: &mut CPU) { unimplemented!("ANDn not implemented") }
pub fn RST20(cpu: &mut CPU) { unimplemented!("RST20 not implemented") }

pub fn ADDSPn(cpu: &mut CPU) { unimplemented!("ADDSPn not implemented") }
pub fn JPHL(cpu: &mut CPU) { unimplemented!("JPHL not implemented") }
pub fn LDmmA(cpu: &mut CPU) { unimplemented!("LDmmA not implemented") }

pub fn XORn(cpu: &mut CPU) { unimplemented!("XORn not implemented") }
pub fn RST28(cpu: &mut CPU) { unimplemented!("RST28 not implemented") }

// F0
pub fn LDAIOn(cpu: &mut CPU) { unimplemented!("LDAIOn not implemented") }
pub fn POPAF(cpu: &mut CPU) { unimplemented!("POPAF not implemented") }
pub fn LDAIOC(cpu: &mut CPU) { unimplemented!("LDAIOC not implemented") }
pub fn DI(cpu: &mut CPU) { unimplemented!("DI not implemented") }

pub fn PUSHAF(cpu: &mut CPU) { unimplemented!("PUSHAF not implemented") }
pub fn ORn(cpu: &mut CPU) { unimplemented!("ORn not implemented") }
pub fn RST30(cpu: &mut CPU) { unimplemented!("RST30 not implemented") }

pub fn LDHLSPn(cpu: &mut CPU) { unimplemented!("LDHLSPn not implemented") }
pub fn LDAmm(cpu: &mut CPU) { unimplemented!("LDAmm not implemented") }
pub fn EI(cpu: &mut CPU) { unimplemented!("EI not implemented") }

pub fn CPn(cpu: &mut CPU) { unimplemented!("CPn not implemented") }
pub fn RST38(cpu: &mut CPU) { unimplemented!("RST38 not implemented") }
