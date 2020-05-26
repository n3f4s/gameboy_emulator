#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
use z80::cpu::CPU;
use z80::instructions::*;
use z80::memory::MMU;
use z80::register::RegisterList;

pub type Opcode = usize; // FIXME adapt to the actual number of opcode
pub type Instruction = fn(RegisterList, &mut MMU) -> Register;

pub struct OpcodeMap<'a> {
    pub op_map: [Box<Instruction>; 256],
    pub op_name: [&'a str; 256],
}

impl<'a> OpcodeMap<'a> {
    pub fn new() -> OpcodeMap<'a> {
        OpcodeMap {
            op_map: [
                // 00
                Box::new(noop), Box::new(LDBCnn), Box::new(LDBCmA), Box::new(INCBC),
                Box::new(INCr_b), Box::new(DECr_b), Box::new(LDrn_b), Box::new(RLCA),
                Box::new(LDmmSP), Box::new(ADDHLBC), Box::new(LDABCm), Box::new(DECBC),
                Box::new(INCr_c), Box::new(DECr_c), Box::new(LDrn_c), Box::new(RRCA),
                // 10
                Box::new(DJNZn), Box::new(LDDEnn), Box::new(LDDEmA), Box::new(INCDE),
                Box::new(INCr_d), Box::new(DECr_d), Box::new(LDrn_d), Box::new(RLA),
                Box::new(JRn), Box::new(ADDHLDE), Box::new(LDADEm), Box::new(DECDE),
                Box::new(INCr_e), Box::new(DECr_e), Box::new(LDrn_e), Box::new(RRA),
                // 20
                Box::new(JRNZn), Box::new(LDHLnn), Box::new(LDHLIA), Box::new(INCHL),
                Box::new(INCr_h), Box::new(DECr_h), Box::new(LDrn_h), Box::new(DAA),
                Box::new(JRZn), Box::new(ADDHLHL), Box::new(LDAHLI), Box::new(DECHL),
                Box::new(INCr_l), Box::new(DECr_l), Box::new(LDrn_l), Box::new(CPL),
                // 30
                Box::new(JRNCn), Box::new(LDSPnn), Box::new(LDHLDA), Box::new(INCSP),
                Box::new(INCHLm), Box::new(DECHLm), Box::new(LDHLmn), Box::new(SCF),
                Box::new(JRCn), Box::new(ADDHLSP), Box::new(LDAHLD), Box::new(DECSP),
                Box::new(INCr_a), Box::new(DECr_a), Box::new(LDrn_a), Box::new(CCF),
                // 40
                Box::new(LDrr_bb), Box::new(LDrr_bc), Box::new(LDrr_bd), Box::new(LDrr_be),
                Box::new(LDrr_bh), Box::new(LDrr_bl), Box::new(LDrHLm_b), Box::new(LDrr_ba),
                Box::new(LDrr_cb), Box::new(LDrr_cc), Box::new(LDrr_cd), Box::new(LDrr_ce),
                Box::new(LDrr_ch), Box::new(LDrr_cl), Box::new(LDrHLm_c), Box::new(LDrr_ca),
                // 50
                Box::new(LDrr_db), Box::new(LDrr_dc), Box::new(LDrr_dd), Box::new(LDrr_de),
                Box::new(LDrr_dh), Box::new(LDrr_dl), Box::new(LDrHLm_d), Box::new(LDrr_da),
                Box::new(LDrr_eb), Box::new(LDrr_ec), Box::new(LDrr_ed), Box::new(LDrr_ee),
                Box::new(LDrr_eh), Box::new(LDrr_el), Box::new(LDrHLm_e), Box::new(LDrr_ea),
                // 60
                Box::new(LDrr_hb), Box::new(LDrr_hc), Box::new(LDrr_hd), Box::new(LDrr_he),
                Box::new(LDrr_hh), Box::new(LDrr_hl), Box::new(LDrHLm_h), Box::new(LDrr_ha),
                Box::new(LDrr_lb), Box::new(LDrr_lc), Box::new(LDrr_ld), Box::new(LDrr_le),
                Box::new(LDrr_lh), Box::new(LDrr_ll), Box::new(LDrHLm_l), Box::new(LDrr_la),
                // 70
                Box::new(LDHLmr_b), Box::new(LDHLmr_c), Box::new(LDHLmr_d), Box::new(LDHLmr_e),
                Box::new(LDHLmr_h), Box::new(LDHLmr_l), Box::new(HALT), Box::new(LDHLmr_a),
                Box::new(LDrr_ab), Box::new(LDrr_ac), Box::new(LDrr_ad), Box::new(LDrr_ae),
                Box::new(LDrr_ah), Box::new(LDrr_al), Box::new(LDrHLm_a), Box::new(LDrr_aa),
                // 80
                Box::new(ADDr_b), Box::new(ADDr_c), Box::new(ADDr_d), Box::new(ADDr_e),
                Box::new(ADDr_h), Box::new(ADDr_l), Box::new(ADDHL), Box::new(ADDr_a),
                Box::new(ADCr_b), Box::new(ADCr_c), Box::new(ADCr_d), Box::new(ADCr_e),
                Box::new(ADCr_h), Box::new(ADCr_l), Box::new(ADCHL), Box::new(ADCr_a),
                // 90
                Box::new(SUBr_b), Box::new(SUBr_c), Box::new(SUBr_d), Box::new(SUBr_e),
                Box::new(SUBr_h), Box::new(SUBr_l), Box::new(SUBHL), Box::new(SUBr_a),
                Box::new(SBCr_b), Box::new(SBCr_c), Box::new(SBCr_d), Box::new(SBCr_e),
                Box::new(SBCr_h), Box::new(SBCr_l), Box::new(SBCHL), Box::new(SBCr_a),
                // A0
                Box::new(ANDr_b), Box::new(ANDr_c), Box::new(ANDr_d), Box::new(ANDr_e),
                Box::new(ANDr_h), Box::new(ANDr_l), Box::new(ANDHL), Box::new(ANDr_a),
                Box::new(XORr_b), Box::new(XORr_c), Box::new(XORr_d), Box::new(XORr_e),
                Box::new(XORr_h), Box::new(XORr_l), Box::new(XORHL), Box::new(XORr_a),
                // B0
                Box::new(ORr_b), Box::new(ORr_c), Box::new(ORr_d), Box::new(ORr_e),
                Box::new(ORr_h), Box::new(ORr_l), Box::new(ORHL), Box::new(ORr_a),
                Box::new(CPr_b), Box::new(CPr_c), Box::new(CPr_d), Box::new(CPr_e),
                Box::new(CPr_h), Box::new(CPr_l), Box::new(CPHL), Box::new(CPr_a),
                // C0
                Box::new(RETNZ), Box::new(POPBC), Box::new(JPNZnn), Box::new(JPnn),
                Box::new(CALLNZnn), Box::new(PUSHBC), Box::new(ADDn), Box::new(RST00),
                Box::new(RETZ), Box::new(RET), Box::new(JPZnn), Box::new(MAPcb),
                Box::new(CALLZnn), Box::new(CALLnn), Box::new(ADCn), Box::new(RST08),
                // D0
                Box::new(RETNC), Box::new(POPDE), Box::new(JPNCnn), Box::new(XX),
                Box::new(CALLNCnn), Box::new(PUSHDE), Box::new(SUBn), Box::new(RST10),
                Box::new(RETC), Box::new(RETI), Box::new(JPCnn), Box::new(XX),
                Box::new(CALLCnn), Box::new(XX), Box::new(SBCn), Box::new(RST18),
                // E0
                Box::new(LDIOnA), Box::new(POPHL), Box::new(LDIOCA), Box::new(XX),
                Box::new(XX), Box::new(PUSHHL), Box::new(ANDn), Box::new(RST20),
                Box::new(ADDSPn), Box::new(JPHL), Box::new(LDmmA), Box::new(XX),
                Box::new(XX), Box::new(XX), Box::new(XORn), Box::new(RST28),
                // F0
                Box::new(LDAIOn), Box::new(POPAF), Box::new(LDAIOC), Box::new(DI),
                Box::new(XX), Box::new(PUSHAF), Box::new(ORn), Box::new(RST30),
                Box::new(LDHLSPn), Box::new(XX), Box::new(LDAmm), Box::new(EI),
                Box::new(XX), Box::new(XX), Box::new(CPn), Box::new(RST38),
            ],
            op_name: [
                // 00
                "NOP", "LDBCnn", "LDBCmA", "INCBC",
                "INCr_b", "DECr_b", "LDrn_b", "RLCA",
                "LDmmSP", "ADDHLBC", "LDABCm", "DECBC",
                "INCr_c", "DECr_c", "LDrn_c", "RRCA",
                // 10
                "DJNZn", "LDDEnn", "LDDEmA", "INCDE",
                "INCr_d", "DECr_d", "LDrn_d", "RLA",
                "JRn", "ADDHLDE", "LDADEm", "DECDE",
                "INCr_e", "DECr_e", "LDrn_e", "RRA",
                // 20
                "JRNZn", "LDHLnn", "LDHLIA", "INCHL",
                "INCr_h", "DECr_h", "LDrn_h", "DAA",
                "JRZn", "ADDHLHL", "LDAHLI", "DECHL",
                "INCr_l", "DECr_l", "LDrn_l", "CPL",
                // 30
                "JRNCn", "LDSPnn", "LDHLDA", "INCSP",
                "INCHLm", "DECHLm", "LDHLmn", "SCF",
                "JRCn", "ADDHLSP", "LDAHLD", "DECSP",
                "INCr_a", "DECr_a", "LDrn_a", "CCF",
                // 40
                "LDrr_bb", "LDrr_bc", "LDrr_bd", "LDrr_be",
                "LDrr_bh", "LDrr_bl", "LDrHLm_b", "LDrr_ba",
                "LDrr_cb", "LDrr_cc", "LDrr_cd", "LDrr_ce",
                "LDrr_ch", "LDrr_cl", "LDrHLm_c", "LDrr_ca",
                // 50
                "LDrr_db", "LDrr_dc", "LDrr_dd", "LDrr_de",
                "LDrr_dh", "LDrr_dl", "LDrHLm_d", "LDrr_da",
                "LDrr_eb", "LDrr_ec", "LDrr_ed", "LDrr_ee",
                "LDrr_eh", "LDrr_el", "LDrHLm_e", "LDrr_ea",
                // 60
                "LDrr_hb", "LDrr_hc", "LDrr_hd", "LDrr_he",
                "LDrr_hh", "LDrr_hl", "LDrHLm_h", "LDrr_ha",
                "LDrr_lb", "LDrr_lc", "LDrr_ld", "LDrr_le",
                "LDrr_lh", "LDrr_ll", "LDrHLm_l", "LDrr_la",
                // 70
                "LDHLmr_b", "LDHLmr_c", "LDHLmr_d", "LDHLmr_e",
                "LDHLmr_h", "LDHLmr_l", "HALT", "LDHLmr_a",
                "LDrr_ab", "LDrr_ac", "LDrr_ad", "LDrr_ae",
                "LDrr_ah", "LDrr_al", "LDrHLm_a", "LDrr_aa",
                // 80
                "ADDr_b", "ADDr_c", "ADDr_d", "ADDr_e",
                "ADDr_h", "ADDr_l", "ADDHL", "ADDr_a",
                "ADCr_b", "ADCr_c", "ADCr_d", "ADCr_e",
                "ADCr_h", "ADCr_l", "ADCHL", "ADCr_a",
                // 90
                "SUBr_b", "SUBr_c", "SUBr_d", "SUBr_e",
                "SUBr_h", "SUBr_l", "SUBHL", "SUBr_a",
                "SBCr_b", "SBCr_c", "SBCr_d", "SBCr_e",
                "SBCr_h", "SBCr_l", "SBCHL", "SBCr_a",
                // A0
                "ANDr_b", "ANDr_c", "ANDr_d", "ANDr_e",
                "ANDr_h", "ANDr_l", "ANDHL", "ANDr_a",
                "XORr_b", "XORr_c", "XORr_d", "XORr_e",
                "XORr_h", "XORr_l", "XORHL", "XORr_a",
                // B0
                "ORr_b", "ORr_c", "ORr_d", "ORr_e",
                "ORr_h", "ORr_l", "ORHL", "ORr_a",
                "CPr_b", "CPr_c", "CPr_d", "CPr_e",
                "CPr_h", "CPr_l", "CPHL", "CPr_a",
                // C0
                "RETNZ", "POPBC", "JPNZnn", "JPnn",
                "CALLNZnn", "PUSHBC", "ADDn", "RST00",
                "RETZ", "RET", "JPZnn", "MAPcb",
                "CALLZnn", "CALLnn", "ADCn", "RST08",
                // D0
                "RETNC", "POPDE", "JPNCnn", "XX",
                "CALLNCnn", "PUSHDE", "SUBn", "RST10",
                "RETC", "RETI", "JPCnn", "XX",
                "CALLCnn", "XX", "SBCn", "RST18",
                // E0
                "LDIOnA", "POPHL", "LDIOCA", "XX",
                "XX", "PUSHHL", "ANDn", "RST20",
                "ADDSPn", "JPHL", "LDmmA", "XX",
                "XX", "XX", "XORn", "RST28",
                // F0
                "LDAIOn", "POPAF", "LDAIOC", "DI",
                "XX", "PUSHAF", "ORn", "RST30",
                "LDHLSPn", "XX", "LDAmm", "EI",
                "XX", "XX", "CPn", "RST38",
            ],
        }
    }
}
