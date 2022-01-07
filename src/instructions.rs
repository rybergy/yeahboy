use std::fmt::Display;

use crate::{bits, mmu::Mmu};

pub use opcodes::UNPREFIXED_INSTRUCTIONS;

mod opcodes {

    use super::IAction::*;
    use super::IFlag::*;
    use super::ILocation::*;
    use super::IRegister::{C, *};
    use super::Instruction;

    lazy_static! {
        pub static ref UNPREFIXED_INSTRUCTIONS: Vec<Instruction> = vec![
            Instruction::new(NOP, 1, 4), // 0x00
            Instruction::new(LD16(Register(BC), ImmediateWord), 3, 12), // 0x01
            Instruction::new(LD(RegisterIndirectByte(BC), Register(A)), 1, 8), // 0x02
            Instruction::new(INC16(Register(BC)), 1, 8), // 0x03
            Instruction::new(INC(Register(B)), 1, 4), // 0x04
            Instruction::new(DEC(Register(B)), 1, 4), // 0x05
            Instruction::new(LD(Register(B), ImmediateByte), 2, 8), // 0x06
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x07
            Instruction::new(LD16(ImmediateWordIndirectWord, Register(SP)), 3, 20), // 0x08
            Instruction::new(ADD16(Register(HL), Register(BC)), 1, 8), // 0x09
            Instruction::new(LD(Register(A), RegisterIndirectByte(BC)), 1, 8), // 0x0a
            Instruction::new(DEC16(Register(BC)), 1, 8), // 0x0b
            Instruction::new(INC(Register(C)), 1, 4), // 0x0c
            Instruction::new(DEC(Register(C)), 1, 4), // 0x0d
            Instruction::new(LD(Register(C), ImmediateByte), 2, 8), // 0x0e
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x0f
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x10
            Instruction::new(LD16(Register(DE), ImmediateWord), 3, 12), // 0x11
            Instruction::new(LD(RegisterIndirectByte(DE), Register(A)), 1, 8), // 0x12
            Instruction::new(INC16(Register(DE)), 1, 8), // 0x13
            Instruction::new(INC(Register(D)), 1, 4), // 0x14
            Instruction::new(DEC(Register(D)), 1, 4), // 0x15
            Instruction::new(LD(Register(D), ImmediateByte), 2, 8), // 0x16
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x17
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0x18
            Instruction::new(ADD16(Register(HL), Register(DE)), 1, 8), // 0x19
            Instruction::new(LD(Register(A), RegisterIndirectByte(DE)), 1, 8), // 0x1a
            Instruction::new(DEC16(Register(DE)), 1, 8), // 0x1b
            Instruction::new(INC(Register(E)), 1, 4), // 0x1c
            Instruction::new(DEC(Register(E)), 1, 4), // 0x1d
            Instruction::new(LD(Register(E), ImmediateByte), 2, 8), // 0x1e
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x1f
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0x20
            Instruction::new(LD16(Register(HL), ImmediateWord), 3, 12), // 0x21
            Instruction::new(LD(RegisterIndirectByte(HL_INC), Register(A)), 1, 8), // 0x22
            Instruction::new(INC16(Register(HL)), 1, 8), // 0x23
            Instruction::new(INC(Register(H)), 1, 4), // 0x24
            Instruction::new(DEC(Register(H)), 1, 4), // 0x25
            Instruction::new(LD(Register(H), ImmediateByte), 2, 8), // 0x26
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x27
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0x28
            Instruction::new(ADD16(Register(HL), Register(HL)), 1, 8), // 0x29
            Instruction::new(LD(Register(A), RegisterIndirectByte(HL_INC)), 1, 8), // 0x2a
            Instruction::new(DEC16(Register(HL)), 1, 8), // 0x2b
            Instruction::new(INC(Register(L)), 1, 4), // 0x2c
            Instruction::new(DEC(Register(L)), 1, 4), // 0x2d
            Instruction::new(LD(Register(L), ImmediateByte), 2, 8), // 0x2e
            Instruction::new(CPL, 1, 4), // 0x2f
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0x30
            Instruction::new(LD16(Register(SP), ImmediateWord), 3, 12), // 0x31
            Instruction::new(LD(RegisterIndirectByte(HL_DEC), Register(A)), 1, 8), // 0x32
            Instruction::new(INC16(Register(SP)), 1, 8), // 0x33
            Instruction::new(INC(RegisterIndirectByte(HL)), 1, 12), // 0x34
            Instruction::new(DEC(RegisterIndirectByte(HL)), 1, 12), // 0x35
            Instruction::new(LD(RegisterIndirectByte(HL), ImmediateByte), 2, 12), // 0x36
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x37
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0x38
            Instruction::new(ADD16(Register(HL), Register(SP)), 1, 8), // 0x39
            Instruction::new(LD(Register(A), RegisterIndirectByte(HL_DEC)), 1, 8), // 0x3a
            Instruction::new(DEC16(Register(SP)), 1, 8), // 0x3b
            Instruction::new(INC(Register(A)), 1, 4), // 0x3c
            Instruction::new(DEC(Register(A)), 1, 4), // 0x3d
            Instruction::new(LD(Register(A), ImmediateByte), 2, 8), // 0x3e
            Instruction::new(CCF, 1, 4), // 0x3f
            Instruction::new(LD(Register(B), Register(B)), 1, 4), // 0x40
            Instruction::new(LD(Register(B), Register(C)), 1, 4), // 0x41
            Instruction::new(LD(Register(B), Register(D)), 1, 4), // 0x42
            Instruction::new(LD(Register(B), Register(E)), 1, 4), // 0x43
            Instruction::new(LD(Register(B), Register(H)), 1, 4), // 0x44
            Instruction::new(LD(Register(B), Register(L)), 1, 4), // 0x45
            Instruction::new(LD(Register(B), RegisterIndirectByte(HL)), 1, 8), // 0x46
            Instruction::new(LD(Register(B), Register(A)), 1, 4), // 0x47
            Instruction::new(LD(Register(C), Register(B)), 1, 4), // 0x48
            Instruction::new(LD(Register(C), Register(C)), 1, 4), // 0x49
            Instruction::new(LD(Register(C), Register(D)), 1, 4), // 0x4a
            Instruction::new(LD(Register(C), Register(E)), 1, 4), // 0x4b
            Instruction::new(LD(Register(C), Register(H)), 1, 4), // 0x4c
            Instruction::new(LD(Register(C), Register(L)), 1, 4), // 0x4d
            Instruction::new(LD(Register(C), RegisterIndirectByte(HL)), 1, 8), // 0x4e
            Instruction::new(LD(Register(C), Register(A)), 1, 4), // 0x4f
            Instruction::new(LD(Register(D), Register(B)), 1, 4), // 0x50
            Instruction::new(LD(Register(D), Register(C)), 1, 4), // 0x51
            Instruction::new(LD(Register(D), Register(D)), 1, 4), // 0x52
            Instruction::new(LD(Register(D), Register(E)), 1, 4), // 0x53
            Instruction::new(LD(Register(D), Register(H)), 1, 4), // 0x54
            Instruction::new(LD(Register(D), Register(L)), 1, 4), // 0x55
            Instruction::new(LD(Register(D), RegisterIndirectByte(HL)), 1, 8), // 0x56
            Instruction::new(LD(Register(D), Register(A)), 1, 4), // 0x57
            Instruction::new(LD(Register(E), Register(B)), 1, 4), // 0x58
            Instruction::new(LD(Register(E), Register(C)), 1, 4), // 0x59
            Instruction::new(LD(Register(E), Register(D)), 1, 4), // 0x5a
            Instruction::new(LD(Register(E), Register(E)), 1, 4), // 0x5b
            Instruction::new(LD(Register(E), Register(H)), 1, 4), // 0x5c
            Instruction::new(LD(Register(E), Register(L)), 1, 4), // 0x5d
            Instruction::new(LD(Register(E), RegisterIndirectByte(HL)), 1, 8), // 0x5e
            Instruction::new(LD(Register(E), Register(A)), 1, 4), // 0x5f
            Instruction::new(LD(Register(H), Register(B)), 1, 4), // 0x60
            Instruction::new(LD(Register(H), Register(C)), 1, 4), // 0x61
            Instruction::new(LD(Register(H), Register(D)), 1, 4), // 0x62
            Instruction::new(LD(Register(H), Register(E)), 1, 4), // 0x63
            Instruction::new(LD(Register(H), Register(H)), 1, 4), // 0x64
            Instruction::new(LD(Register(H), Register(L)), 1, 4), // 0x65
            Instruction::new(LD(Register(H), RegisterIndirectByte(HL)), 1, 8), // 0x66
            Instruction::new(LD(Register(H), Register(A)), 1, 4), // 0x67
            Instruction::new(LD(Register(L), Register(B)), 1, 4), // 0x68
            Instruction::new(LD(Register(L), Register(C)), 1, 4), // 0x69
            Instruction::new(LD(Register(L), Register(D)), 1, 4), // 0x6a
            Instruction::new(LD(Register(L), Register(E)), 1, 4), // 0x6b
            Instruction::new(LD(Register(L), Register(H)), 1, 4), // 0x6c
            Instruction::new(LD(Register(L), Register(L)), 1, 4), // 0x6d
            Instruction::new(LD(Register(L), RegisterIndirectByte(HL)), 1, 8), // 0x6e
            Instruction::new(LD(Register(L), Register(A)), 1, 4), // 0x6f
            Instruction::new(LD(RegisterIndirectByte(HL), Register(B)), 1, 8), // 0x70
            Instruction::new(LD(RegisterIndirectByte(HL), Register(C)), 1, 8), // 0x71
            Instruction::new(LD(RegisterIndirectByte(HL), Register(D)), 1, 8), // 0x72
            Instruction::new(LD(RegisterIndirectByte(HL), Register(E)), 1, 8), // 0x73
            Instruction::new(LD(RegisterIndirectByte(HL), Register(H)), 1, 8), // 0x74
            Instruction::new(LD(RegisterIndirectByte(HL), Register(L)), 1, 8), // 0x75
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0x76
            Instruction::new(LD(RegisterIndirectByte(HL), Register(A)), 1, 8), // 0x77
            Instruction::new(LD(Register(A), Register(B)), 1, 4), // 0x78
            Instruction::new(LD(Register(A), Register(C)), 1, 4), // 0x79
            Instruction::new(LD(Register(A), Register(D)), 1, 4), // 0x7a
            Instruction::new(LD(Register(A), Register(E)), 1, 4), // 0x7b
            Instruction::new(LD(Register(A), Register(H)), 1, 4), // 0x7c
            Instruction::new(LD(Register(A), Register(L)), 1, 4), // 0x7d
            Instruction::new(LD(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0x7e
            Instruction::new(LD(Register(A), Register(A)), 1, 4), // 0x7f
            Instruction::new(ADD(Register(A), Register(B)), 1, 4), // 0x80
            Instruction::new(ADD(Register(A), Register(C)), 1, 4), // 0x81
            Instruction::new(ADD(Register(A), Register(D)), 1, 4), // 0x82
            Instruction::new(ADD(Register(A), Register(E)), 1, 4), // 0x83
            Instruction::new(ADD(Register(A), Register(H)), 1, 4), // 0x84
            Instruction::new(ADD(Register(A), Register(L)), 1, 4), // 0x85
            Instruction::new(ADD(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0x86
            Instruction::new(ADD(Register(A), Register(A)), 1, 4), // 0x87
            Instruction::new(ADD(Register(A), Register(B)), 1, 4), // 0x88
            Instruction::new(ADC(Register(A), Register(C)), 1, 4), // 0x89
            Instruction::new(ADC(Register(A), Register(D)), 1, 4), // 0x8a
            Instruction::new(ADC(Register(A), Register(E)), 1, 4), // 0x8b
            Instruction::new(ADC(Register(A), Register(H)), 1, 4), // 0x8c
            Instruction::new(ADC(Register(A), Register(L)), 1, 4), // 0x8d
            Instruction::new(ADC(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0x8e
            Instruction::new(ADC(Register(A), Register(A)), 1, 4), // 0x8f
            Instruction::new(SUB(Register(A), Register(B)), 1, 4), // 0x90
            Instruction::new(SUB(Register(A), Register(C)), 1, 4), // 0x91
            Instruction::new(SUB(Register(A), Register(D)), 1, 4), // 0x92
            Instruction::new(SUB(Register(A), Register(E)), 1, 4), // 0x93
            Instruction::new(SUB(Register(A), Register(H)), 1, 4), // 0x94
            Instruction::new(SUB(Register(A), Register(L)), 1, 4), // 0x95
            Instruction::new(SUB(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0x96
            Instruction::new(SUB(Register(A), Register(A)), 1, 4), // 0x97
            Instruction::new(SUB(Register(A), Register(B)), 1, 4), // 0x98
            Instruction::new(SBC(Register(A), Register(C)), 1, 4), // 0x99
            Instruction::new(SBC(Register(A), Register(D)), 1, 4), // 0x9a
            Instruction::new(SBC(Register(A), Register(E)), 1, 4), // 0x9b
            Instruction::new(SBC(Register(A), Register(H)), 1, 4), // 0x9c
            Instruction::new(SBC(Register(A), Register(L)), 1, 4), // 0x9d
            Instruction::new(SBC(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0x9e
            Instruction::new(SBC(Register(A), Register(A)), 1, 4), // 0x9f
            Instruction::new(AND(Register(A), Register(B)), 1, 4), // 0xa0
            Instruction::new(AND(Register(A), Register(C)), 1, 4), // 0xa1
            Instruction::new(AND(Register(A), Register(D)), 1, 4), // 0xa2
            Instruction::new(AND(Register(A), Register(E)), 1, 4), // 0xa3
            Instruction::new(AND(Register(A), Register(H)), 1, 4), // 0xa4
            Instruction::new(AND(Register(A), Register(L)), 1, 4), // 0xa5
            Instruction::new(AND(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0xa6
            Instruction::new(AND(Register(A), Register(A)), 1, 4), // 0xa7
            Instruction::new(AND(Register(A), Register(B)), 1, 4), // 0xa8
            Instruction::new(XOR(Register(A), Register(C)), 1, 4), // 0xa9
            Instruction::new(XOR(Register(A), Register(D)), 1, 4), // 0xaa
            Instruction::new(XOR(Register(A), Register(E)), 1, 4), // 0xab
            Instruction::new(XOR(Register(A), Register(H)), 1, 4), // 0xac
            Instruction::new(XOR(Register(A), Register(L)), 1, 4), // 0xad
            Instruction::new(XOR(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0xae
            Instruction::new(XOR(Register(A), Register(A)), 1, 4), // 0xaf
            Instruction::new(OR(Register(A), Register(B)), 1, 4), // 0xb0
            Instruction::new(OR(Register(A), Register(C)), 1, 4), // 0xb1
            Instruction::new(OR(Register(A), Register(D)), 1, 4), // 0xb2
            Instruction::new(OR(Register(A), Register(E)), 1, 4), // 0xb3
            Instruction::new(OR(Register(A), Register(H)), 1, 4), // 0xb4
            Instruction::new(OR(Register(A), Register(L)), 1, 4), // 0xb5
            Instruction::new(OR(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0xb6
            Instruction::new(OR(Register(A), Register(A)), 1, 4), // 0xb7
            Instruction::new(OR(Register(A), Register(B)), 1, 4), // 0xb8
            Instruction::new(CP(Register(A), Register(C)), 1, 4), // 0xb9
            Instruction::new(CP(Register(A), Register(D)), 1, 4), // 0xba
            Instruction::new(CP(Register(A), Register(E)), 1, 4), // 0xbb
            Instruction::new(CP(Register(A), Register(H)), 1, 4), // 0xbc
            Instruction::new(CP(Register(A), Register(L)), 1, 4), // 0xbd
            Instruction::new(CP(Register(A), RegisterIndirectByte(HL)), 1, 8), // 0xbe
            Instruction::new(CP(Register(A), Register(A)), 1, 4), // 0xbf
            Instruction::new(UNIMPLEMENTED, 1, 20), // 0xc0
            Instruction::new(UNIMPLEMENTED, 1, 12), // 0xc1
            Instruction::new(UNIMPLEMENTED, 3, 16), // 0xc2
            Instruction::new(JP(TRUE, ImmediateWord), 3, 16), // 0xc3
            Instruction::new(UNIMPLEMENTED, 3, 24), // 0xc4
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xc5
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xc6
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xc7
            Instruction::new(UNIMPLEMENTED, 1, 20), // 0xc8
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xc9
            Instruction::new(UNIMPLEMENTED, 3, 16), // 0xca
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0xcb
            Instruction::new(UNIMPLEMENTED, 3, 24), // 0xcc
            Instruction::new(UNIMPLEMENTED, 3, 24), // 0xcd
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xce
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xcf
            Instruction::new(UNIMPLEMENTED, 1, 20), // 0xd0
            Instruction::new(UNIMPLEMENTED, 1, 12), // 0xd1
            Instruction::new(UNIMPLEMENTED, 3, 16), // 0xd2
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xd3: NOT REAL
            Instruction::new(UNIMPLEMENTED, 3, 24), // 0xd4
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xd5
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xd6
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xd7
            Instruction::new(UNIMPLEMENTED, 1, 20), // 0xd8
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xd9
            Instruction::new(UNIMPLEMENTED, 3, 16), // 0xda
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xdb: NOT REAL
            Instruction::new(UNIMPLEMENTED, 3, 24), // 0xdc
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xdd: NOT REAL
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xde
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xdf
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0xe0
            Instruction::new(UNIMPLEMENTED, 1, 12), // 0xe1
            Instruction::new(UNIMPLEMENTED, 1, 8), // 0xe2
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xe3: NOT REAL
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xe4: NOT REAL
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xe5
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xe6
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xe7
            Instruction::new(UNIMPLEMENTED, 2, 16), // 0xe8
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0xe9
            Instruction::new(UNIMPLEMENTED, 3, 16), // 0xea
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xeb: NOT REAL
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xec: NOT REAL
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xed: NOT REAL
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xee
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xef
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0xf0
            Instruction::new(UNIMPLEMENTED, 1, 12), // 0xf1
            Instruction::new(UNIMPLEMENTED, 1, 8), // 0xf2
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0xf3
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xf4: NOT REAL
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xf5
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xf6
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xf7
            Instruction::new(UNIMPLEMENTED, 2, 12), // 0xf8
            Instruction::new(UNIMPLEMENTED, 1, 8), // 0xf9
            Instruction::new(UNIMPLEMENTED, 3, 16), // 0xfa
            Instruction::new(UNIMPLEMENTED, 1, 4), // 0xfb
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xfc: NOT REAL
            Instruction::new(UNIMPLEMENTED, 1, 0), // 0xfd: NOT REAL
            Instruction::new(UNIMPLEMENTED, 2, 8), // 0xfe
            Instruction::new(UNIMPLEMENTED, 1, 16), // 0xff
        ];
    }
}

#[derive(Copy, Clone)]
pub enum IFlag {
    /// Represents no flag needed for this action; always evaluates true
    TRUE,
    Z,
    NZ,
    CY,
    NCY,
}

impl Display for IFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IFlag::TRUE => "",
                IFlag::Z => "Z",
                IFlag::NZ => "NZ",
                IFlag::CY => "CY",
                IFlag::NCY => "NCY",
            }
        )
    }
}

#[derive(Copy, Clone)]
pub enum IRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    /// Read HL then increment (HL+)
    HL_INC,
    /// Read HL then decrement (HL-)
    HL_DEC,
    SP,
    // PC is a register, but not used in any instructions
}

impl Display for IRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IRegister::A => "A",
                IRegister::B => "B",
                IRegister::C => "C",
                IRegister::D => "D",
                IRegister::E => "E",
                IRegister::H => "H",
                IRegister::L => "L",
                IRegister::AF => "AF",
                IRegister::BC => "BC",
                IRegister::DE => "DE",
                IRegister::HL => "HL",
                IRegister::HL_INC => "HL+",
                IRegister::HL_DEC => "HL-",
                IRegister::SP => "SP",
            },
        )
    }
}

#[derive(Copy, Clone)]
pub enum ILocation {
    /// The direct register
    Register(IRegister),

    /// The memory address pointed to by the register (byte)
    RegisterIndirectByte(IRegister),

    /// The memory address pointed to by the register (word)
    RegisterIndirectWord(IRegister),

    /// The immediate operand (byte)
    ImmediateByte,

    /// The immediate operand (word)
    ImmediateWord,

    /// The memory address pointed to by the immediate operand (byte)
    ImmediateByteIndirectByte,

    /// The memory address pointed to by the immediate operand (byte)
    ImmediateByteIndirectWord,

    /// The memory address pointed to by the immediate operand (word)
    ImmediateWordIndirectByte,

    /// The memory address pointed to by the immediate operand (word)
    ImmediateWordIndirectWord,
    // /// The immediate operand (signed byte)
    // ImmediateSignedByte,
}

impl Display for ILocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ILocation::Register(reg) => write!(f, "{}", reg),
            ILocation::RegisterIndirectByte(reg) | ILocation::RegisterIndirectWord(reg) => {
                write!(f, "({})", reg)
            }
            ILocation::ImmediateByte => write!(f, "u8"),
            ILocation::ImmediateWord => write!(f, "u16"),
            ILocation::ImmediateByteIndirectByte => write!(f, "(u8)"),
            ILocation::ImmediateByteIndirectWord => write!(f, "(u8)"),
            ILocation::ImmediateWordIndirectByte => write!(f, "(u16)"),
            ILocation::ImmediateWordIndirectWord => write!(f, "(u16)"),
            // ILocation::ImmediateSignedByte => write!(f, "i8"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum IAction {
    NOP,
    LD(ILocation, ILocation),
    LD16(ILocation, ILocation),
    JP(IFlag, ILocation),
    CPL,
    CCF,
    INC(ILocation),
    INC16(ILocation),
    DEC(ILocation),
    DEC16(ILocation),
    ADD(ILocation, ILocation),
    ADD16(ILocation, ILocation),
    ADC(ILocation, ILocation),
    SUB(ILocation, ILocation),
    AND(ILocation, ILocation),
    SBC(ILocation, ILocation),
    XOR(ILocation, ILocation),
    OR(ILocation, ILocation),
    CP(ILocation, ILocation),
    UNIMPLEMENTED,
}

impl Display for IAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IAction::NOP => write!(f, "NOP"),
            IAction::LD(dst, src) => write!(f, "LD {}, {}", dst, src),
            IAction::LD16(dst, src) => write!(f, "LD {}, {}", dst, src),
            IAction::INC(loc) => write!(f, "INC {}", loc),
            IAction::INC16(loc) => write!(f, "INC {}", loc),
            IAction::DEC(loc) => write!(f, "DEC {}", loc),
            IAction::DEC16(loc) => write!(f, "DEC {}", loc),
            IAction::JP(flag, loc) => write!(f, "JP {}, {}", flag, loc),
            IAction::CPL => write!(f, "CPL (FLIP A)"),
            IAction::CCF => write!(f, "CCF (FLIP CY)"),
            IAction::UNIMPLEMENTED => write!(f, "<???>"),
            IAction::ADD(dst, src) => write!(f, "ADD {}, {}", dst, src),
            IAction::ADD16(dst, src) => write!(f, "ADD {}, {}", dst, src),
            IAction::ADC(dst, src) => write!(f, "ADC {}, {}", dst, src),
            IAction::SUB(dst, src) => write!(f, "SUB {}, {}", dst, src),
            IAction::SBC(dst, src) => write!(f, "SBC {}, {}", dst, src),
            IAction::AND(dst, src) => write!(f, "AND {}, {}", dst, src),
            IAction::XOR(dst, src) => write!(f, "XOR {}, {}", dst, src),
            IAction::OR(dst, src) => write!(f, "OR {}, {}", dst, src),
            IAction::CP(dst, src) => write!(f, "CP {}, {}", dst, src),
            _ => write!(f, "<display not impl.>"),
        }
    }
}

pub struct Instruction {
    pub action: IAction,
    pub length: u8,
    pub cycles: u8,
}

impl Instruction {
    pub fn new(action: IAction, length: u8, cycles: u8) -> Self {
        Self {
            action,
            length,
            cycles,
        }
    }
}

pub struct DecodedInstruction {
    action: IAction,
    len: u8,
    cycles: u8,

    raw_bytes: Vec<u8>,
}

impl DecodedInstruction {
    pub fn decode(mmu: &Mmu, pc: usize) -> Self {
        let opcode = mmu.rb(pc);

        let instruction = &UNPREFIXED_INSTRUCTIONS[opcode as usize];

        let num_operands = instruction.length - 1;

        // TODO: remove operands, keep only raw bytes
        let mut operands = Vec::with_capacity(instruction.length as usize - 1);
        let mut raw_bytes = Vec::with_capacity(instruction.length as usize);

        raw_bytes.push(opcode);

        for i in 1..=num_operands {
            let byte = mmu.rb(pc + i as usize);
            operands.push(byte);
            raw_bytes.push(byte);
        }

        Self {
            action: instruction.action,
            len: instruction.length,
            cycles: instruction.cycles,
            raw_bytes,
        }
    }

    pub fn action(&self) -> IAction {
        self.action
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn cycles(&self) -> u8 {
        self.cycles
    }

    pub fn opcode(&self) -> u8 {
        self.raw_bytes[0]
    }

    pub fn raw_bytes(&self) -> &Vec<u8> {
        &self.raw_bytes
    }

    fn operands(&self) -> &[u8] {
        &self.raw_bytes[1..]
    }

    pub fn operands_as_u16(&self) -> u16 {
        let operands = self.operands();

        if operands.len() != 2 {
            log::error!("Expected 2 operands for u16, got {}!", operands.len());
        }

        // Pack LE into u16
        bits::pack_u16(operands[1], operands[0])
    }

    pub fn operands_as_u8(&self) -> u8 {
        let operands = self.operands();

        if operands.len() != 1 {
            log::error!("Expected 1 operands for u8, got {:?}!", operands);
        }

        operands[0]
    }
}
