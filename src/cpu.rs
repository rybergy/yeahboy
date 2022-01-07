use crate::{
    bits,
    instructions::{DecodedInstruction, IAction, IFlag, ILocation, IRegister},
    mmu::Mmu,
};
use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const ZERO = 0b1000_0000;
        const SUBTRACT = 0b0100_0000;
        const HALF_CARRY = 0b0010_0000;
        const CARRY = 0b0001_0000;
    }
}

impl std::convert::From<u8> for Flags {
    fn from(n: u8) -> Self {
        Flags::from_bits_truncate(n)
    }
}

impl std::convert::From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        flags.bits
    }
}

#[derive(Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn c(&self) -> u8 {
        self.c
    }

    pub fn d(&self) -> u8 {
        self.d
    }

    pub fn e(&self) -> u8 {
        self.e
    }

    pub fn f(&self) -> Flags {
        self.f
    }

    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn l(&self) -> u8 {
        self.l
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value
    }

    pub fn set_f(&mut self, value: Flags) {
        self.f = value
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value
    }

    pub fn af(&self) -> u16 {
        bits::pack_u16(self.a, self.f.into())
    }

    pub fn bc(&self) -> u16 {
        bits::pack_u16(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        bits::pack_u16(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        bits::pack_u16(self.h, self.l)
    }

    pub fn set_af(&mut self, value: u16) {
        let (a, f) = bits::unpack_u16(value);
        self.a = a;
        self.f = f.into();
    }

    pub fn set_bc(&mut self, value: u16) {
        let (b, c) = bits::unpack_u16(value);
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, value: u16) {
        let (d, e) = bits::unpack_u16(value);
        self.d = d;
        self.e = e;
    }

    pub fn set_hl(&mut self, value: u16) {
        let (h, l) = bits::unpack_u16(value);
        self.h = h;
        self.l = l;
    }

    pub fn has_flag(&self, flag: Flags) -> bool {
        self.f.contains(flag)
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            self.f = self.f | flag
        } else {
            self.f = self.f & (!flag)
        }
    }

    // pub fn unset_flag(&mut self, flag: Flags) {
    //     self.f = self.f & (!flag)
    // }

    pub fn flip_flag(&mut self, flag: Flags) {
        self.f = self.f ^ flag
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0.into(),
            h: 0x01,
            l: 0x4D,
        }
    }
}

pub struct Cpu {
    reg: Registers,
    mmu: Mmu,
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            reg: Registers::new(),
            mmu: Mmu::new(rom),
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn reg(&self) -> &Registers {
        &self.reg
    }

    pub fn mmu(&self) -> &Mmu {
        &self.mmu
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    fn read_register(&mut self, reg: IRegister) -> u8 {
        match reg {
            IRegister::A => self.reg.a(),
            IRegister::B => self.reg.b(),
            IRegister::C => self.reg.c(),
            IRegister::D => self.reg.d(),
            IRegister::E => self.reg.e(),
            IRegister::H => self.reg.h(),
            IRegister::L => self.reg.l(),
            IRegister::AF
            | IRegister::BC
            | IRegister::DE
            | IRegister::HL
            | IRegister::HL_INC
            | IRegister::HL_DEC
            | IRegister::SP => {
                panic!("attempted to read 16-bit register {} as a u8!", reg)
            }
        }
    }

    fn read_register_u16(&mut self, reg: IRegister) -> u16 {
        match reg {
            IRegister::AF => self.reg.af(),
            IRegister::BC => self.reg.bc(),
            IRegister::DE => self.reg.de(),
            IRegister::HL => self.reg.hl(),
            IRegister::HL_INC => {
                let value = self.reg.hl() as u16;
                self.reg.set_hl(value + 1);
                value
            }
            IRegister::HL_DEC => {
                let value = self.reg.hl() as u16;
                self.reg.set_hl(value - 1);
                value
            }
            // We want to accept 8-bit registers as u16 as well, in case we use them for
            // indirect addressing
            IRegister::SP => self.sp,
            IRegister::A => self.reg.a() as u16,
            IRegister::B => self.reg.b() as u16,
            IRegister::C => self.reg.c() as u16,
            IRegister::D => self.reg.d() as u16,
            IRegister::E => self.reg.e() as u16,
            IRegister::H => self.reg.e() as u16,
            IRegister::L => self.reg.l() as u16,
        }
    }

    fn write_register(&mut self, reg: IRegister, value: u8) {
        match reg {
            IRegister::A => self.reg.set_a(value),
            IRegister::B => self.reg.set_b(value),
            IRegister::C => self.reg.set_c(value),
            IRegister::D => self.reg.set_d(value),
            IRegister::E => self.reg.set_e(value),
            IRegister::H => self.reg.set_h(value),
            IRegister::L => self.reg.set_l(value),
            IRegister::AF
            | IRegister::BC
            | IRegister::DE
            | IRegister::HL
            | IRegister::HL_INC
            | IRegister::HL_DEC
            | IRegister::SP => {
                panic!("attempted to write a u8 to 16-bit register {}", reg)
            }
        }
    }

    fn write_register_u16(&mut self, reg: IRegister, value: u16) {
        match reg {
            IRegister::AF => self.reg.set_af(value),
            IRegister::BC => self.reg.set_bc(value),
            IRegister::DE => self.reg.set_de(value),
            IRegister::HL => self.reg.set_hl(value),
            IRegister::HL_INC => self.reg.set_hl(value.overflowing_add(1).0),
            IRegister::HL_DEC => self.reg.set_hl(value.overflowing_sub(1).0),
            IRegister::SP => self.sp = value,
            IRegister::A
            | IRegister::B
            | IRegister::C
            | IRegister::D
            | IRegister::E
            | IRegister::H
            | IRegister::L => {
                panic!("attempted to write a u16 to 8-bit register {}", reg)
            }
        }
    }

    /// Returns u16 though some may only be u8; will never be more than u16
    fn read_location(&mut self, inst: &DecodedInstruction, loc: ILocation) -> u8 {
        match loc {
            ILocation::Register(reg) => self.read_register(reg),
            ILocation::RegisterIndirectByte(reg) => {
                // Get as u16 to access full addressing space
                let addr = self.read_register_u16(reg);
                self.mmu.rb(addr as usize)
            }
            ILocation::ImmediateByte => inst.operands_as_u8(),
            ILocation::ImmediateByteIndirectByte => self.mmu.rb(inst.operands_as_u8() as usize),
            ILocation::ImmediateWordIndirectByte => self.mmu.rb(inst.operands_as_u16() as usize),
            ILocation::ImmediateWord
            | ILocation::ImmediateByteIndirectWord
            | ILocation::ImmediateWordIndirectWord
            | ILocation::RegisterIndirectWord(_) => {
                panic!("attempted to read 16-bit location {} as a u8!", loc)
            }
        }
    }

    /// Returns u16 though some may only be u8; will never be more than u16
    fn read_location_u16(&mut self, inst: &DecodedInstruction, loc: ILocation) -> u16 {
        match loc {
            ILocation::Register(reg) => self.read_register_u16(reg),
            ILocation::ImmediateWord => inst.operands_as_u16(),
            ILocation::ImmediateByteIndirectWord => self.mmu.rw(inst.operands_as_u8() as usize),
            ILocation::ImmediateWordIndirectWord => self.mmu.rw(inst.operands_as_u16() as usize),
            ILocation::RegisterIndirectWord(reg) => {
                let addr = self.read_register(reg);
                self.mmu.rw(addr as usize)
            }
            ILocation::RegisterIndirectByte(_)
            | ILocation::ImmediateByte
            | ILocation::ImmediateByteIndirectByte
            | ILocation::ImmediateWordIndirectByte => {
                panic!("attempted to read 8-bit location {} as a u16!", loc)
            }
        }
    }

    /// Function signature takes u16, but some may only write u8
    fn write_location(&mut self, inst: &DecodedInstruction, loc: ILocation, value: u8) {
        match loc {
            ILocation::Register(reg) => self.write_register(reg, value),
            ILocation::RegisterIndirectByte(reg) => {
                // Get as u16 to access full addressing space
                let addr = self.read_register_u16(reg);
                self.mmu.wb(addr as usize, value)
            }
            ILocation::ImmediateByteIndirectByte => {
                self.mmu.wb(inst.operands_as_u8() as usize, value)
            }
            ILocation::ImmediateWordIndirectByte => {
                self.mmu.wb(inst.operands_as_u16() as usize, value)
            }
            ILocation::ImmediateByte | ILocation::ImmediateWord => {
                panic!("cannot write to immediate value {}!", loc)
            }
            ILocation::RegisterIndirectWord(_)
            | ILocation::ImmediateByteIndirectWord
            | ILocation::ImmediateWordIndirectWord => {
                panic!("attempted to write a u8 to 16-bit location {}", loc)
            }
        }
    }

    /// Function signature takes u16, but some may only write u8
    fn write_location_u16(&mut self, inst: &DecodedInstruction, loc: ILocation, value: u16) {
        match loc {
            ILocation::Register(reg) => self.write_register_u16(reg, value),
            ILocation::RegisterIndirectWord(reg) => {
                let addr = self.read_register(reg);
                self.mmu.ww(addr as usize, value as u16)
            }
            ILocation::ImmediateByteIndirectWord => {
                self.mmu.ww(inst.operands_as_u8() as usize, value)
            }
            ILocation::ImmediateWordIndirectWord => {
                self.mmu.ww(inst.operands_as_u16() as usize, value)
            }
            ILocation::ImmediateByte | ILocation::ImmediateWord => {
                panic!("Cannot write to immediate value {}!", loc)
            }
            ILocation::RegisterIndirectByte(_)
            | ILocation::ImmediateByteIndirectByte
            | ILocation::ImmediateWordIndirectByte => {
                panic!("attempted to write a u16 to 8-bit location {}", loc)
            }
        }
    }

    fn read_flag(&self, flag: IFlag) -> bool {
        match flag {
            IFlag::TRUE => true,
            IFlag::CY => self.reg.has_flag(Flags::CARRY),
            IFlag::NCY => !self.reg.has_flag(Flags::CARRY),
            IFlag::Z => self.reg.has_flag(Flags::ZERO),
            IFlag::NZ => !self.reg.has_flag(Flags::ZERO),
        }
    }

    fn check_zero(&mut self, value: u16) {
        self.reg.set_flag(Flags::ZERO, value == 0)
    }

    fn nop(&self, _inst: &DecodedInstruction) {
        log::info!("NOP")
    }

    fn ld(&mut self, inst: &DecodedInstruction, dst: ILocation, src: ILocation) {
        let value = self.read_location(inst, src);
        self.write_location(inst, dst, value);
    }

    fn ld16(&mut self, inst: &DecodedInstruction, dst: ILocation, src: ILocation) {
        let value = self.read_location_u16(inst, src);
        self.write_location_u16(inst, dst, value);
    }

    /// JP: Jump to the address specified in `loc` if `flag` is set.
    fn jp(&mut self, inst: &DecodedInstruction, flag: IFlag, loc: ILocation) {
        if self.read_flag(flag) {
            self.pc = self.read_location_u16(inst, loc);
        }
    }

    /// CPL: One's complement (flip) register A
    fn cpl(&mut self) {
        self.reg.set_a(!self.reg.a())
    }

    /// CCF: One's complement (flip) carry flag
    fn ccf(&mut self) {
        self.reg.flip_flag(Flags::CARRY);
        self.reg.set_flag(Flags::SUBTRACT, true);
        self.reg.set_flag(Flags::HALF_CARRY, true);
    }

    /// XOR: XOR dst with src, store in dst
    fn xor(&mut self, inst: &DecodedInstruction, dst: ILocation, src: ILocation) {
        let src_value = self.read_location(inst, src);
        let dst_value = self.read_location(inst, dst);

        let value = src_value ^ dst_value;
        self.write_location(inst, dst, value);

        self.reg.set_flag(Flags::ZERO, value == 0);
        self.reg.set_flag(Flags::SUBTRACT, false);
        self.reg.set_flag(Flags::HALF_CARRY, false);
        self.reg.set_flag(Flags::CARRY, false);
    }

    fn dec(&mut self, inst: &DecodedInstruction, loc: ILocation) {
        let before = self.read_location(inst, loc);
        let (after, overflowed) = before.overflowing_sub(1);
        self.write_location(inst, loc, after);

        self.reg.set_flag(Flags::ZERO, after == 0);
        self.reg.set_flag(Flags::SUBTRACT, true);
        // A half-carry only occurs when we overflow from 0 to maxint
        self.reg.set_flag(Flags::HALF_CARRY, overflowed);
    }

    fn dec16(&mut self, inst: &DecodedInstruction, loc: ILocation) {
        let before = self.read_location_u16(inst, loc);
        let (after, _) = before.overflowing_sub(1);
        self.write_location_u16(inst, loc, after);
        // Don't update flags for 16-bit DEC
    }

    fn inc(&mut self, inst: &DecodedInstruction, loc: ILocation) {
        let before = self.read_location(inst, loc);
        let (after, _) = before.overflowing_add(1);
        self.write_location(inst, loc, after);

        self.reg.set_flag(Flags::ZERO, after == 0);
        self.reg.set_flag(Flags::SUBTRACT, false);
        // A half-carry only occurs when we go from 0xFF to 0x100
        self.reg.set_flag(Flags::HALF_CARRY, before == 0xFF);
    }

    fn inc16(&mut self, inst: &DecodedInstruction, loc: ILocation) {
        let before = self.read_location_u16(inst, loc);
        let (after, _) = before.overflowing_add(1);
        self.write_location_u16(inst, loc, after);
        // Don't update flags for 16-bit INC
    }

    fn execute(&mut self, inst: &DecodedInstruction) {
        match inst.action() {
            IAction::NOP => self.nop(inst),
            IAction::LD(dst, src) => self.ld(inst, dst, src),
            IAction::LD16(dst, src) => self.ld16(inst, dst, src),
            IAction::JP(flag, loc) => self.jp(inst, flag, loc),
            IAction::CPL => self.cpl(),
            IAction::CCF => self.ccf(),
            IAction::XOR(dst, src) => self.xor(inst, dst, src),
            IAction::DEC(loc) => self.dec(inst, loc),
            IAction::DEC16(loc) => self.dec16(inst, loc),
            IAction::INC(loc) => self.inc(inst, loc),
            IAction::INC16(loc) => self.inc16(inst, loc),
            _ => {
                log::debug!("dumping");
                log::debug!("af = {:04x}", self.reg.af());
                log::debug!("bc = {:04x}", self.reg.bc());
                log::debug!("de = {:04x}", self.reg.de());
                log::debug!("hl = {:04x}", self.reg.hl());
                log::debug!("sp = {:04x}", self.sp);
                log::debug!("pc = {:04x}", self.pc);
                todo!("instruction 0x{:02x}", inst.opcode())
            }
        }
    }

    pub fn step(&mut self) {
        let inst = DecodedInstruction::decode(&self.mmu, self.pc as usize);
        log::debug!("executing {:02x}: {}", inst.opcode(), inst.action());
        self.pc += inst.len() as u16;
        self.execute(&inst);
    }
}

#[cfg(test)]
mod test {

    use super::Flags;
    use super::Registers;

    #[test]
    fn flags_to_u8_individual() {
        assert_eq!(Flags::ZERO.bits, u8::from(Flags::ZERO));
        assert_eq!(Flags::SUBTRACT.bits, u8::from(Flags::SUBTRACT));
        assert_eq!(Flags::HALF_CARRY.bits, u8::from(Flags::HALF_CARRY));
        assert_eq!(Flags::CARRY.bits, u8::from(Flags::CARRY));
    }

    #[test]
    fn flags_to_u8_all() {
        let flags = Flags::ZERO | Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY;
        assert_eq!(flags.bits, u8::from(flags));
    }

    #[test]
    fn u8_to_flags_individual() {
        assert!(Flags::from(Flags::ZERO).contains(Flags::ZERO));
        assert!(Flags::from(Flags::SUBTRACT).contains(Flags::SUBTRACT));
        assert!(Flags::from(Flags::HALF_CARRY).contains(Flags::HALF_CARRY));
        assert!(Flags::from(Flags::CARRY).contains(Flags::CARRY));
    }

    #[test]
    fn u8_to_flags_all() {
        let flags = Flags::ZERO | Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY;
        assert!(Flags::from(flags).contains(Flags::ZERO));
        assert!(Flags::from(flags).contains(Flags::SUBTRACT));
        assert!(Flags::from(flags).contains(Flags::HALF_CARRY));
        assert!(Flags::from(flags).contains(Flags::CARRY));
    }

    #[test]
    fn reg_set_unset_flag() {
        let mut reg = Registers::new();
        let all_flags = [
            Flags::ZERO,
            Flags::SUBTRACT,
            Flags::HALF_CARRY,
            Flags::CARRY,
        ];

        for flag in all_flags {
            reg.set_flag(flag, false);
        }

        for flag in all_flags {
            assert!(!reg.has_flag(flag));
        }

        for flag in all_flags {
            reg.set_flag(flag, true)
        }

        for flag in all_flags {
            assert!(reg.has_flag(flag))
        }

        reg.set_flag(Flags::SUBTRACT, false);
        reg.set_flag(Flags::CARRY, false);

        assert!(reg.has_flag(Flags::ZERO));
        assert!(!reg.has_flag(Flags::SUBTRACT));
        assert!(reg.has_flag(Flags::HALF_CARRY));
        assert!(!reg.has_flag(Flags::CARRY));

        reg.set_flag(Flags::SUBTRACT, false);
        reg.set_flag(Flags::CARRY, false);
        reg.set_flag(Flags::ZERO, true);
        reg.set_flag(Flags::HALF_CARRY, true);

        assert!(reg.has_flag(Flags::ZERO));
        assert!(!reg.has_flag(Flags::SUBTRACT));
        assert!(reg.has_flag(Flags::HALF_CARRY));
        assert!(!reg.has_flag(Flags::CARRY));
    }

    #[test]
    fn reg_flip_flag() {
        let mut reg = Registers::new();
        let all_flags = [
            Flags::ZERO,
            Flags::SUBTRACT,
            Flags::HALF_CARRY,
            Flags::CARRY,
        ];

        for flag in all_flags {
            reg.set_flag(flag, false);
        }

        for flag in all_flags {
            assert!(!reg.has_flag(flag));
        }

        for flag in all_flags {
            reg.flip_flag(flag);
        }

        for flag in all_flags {
            assert!(reg.has_flag(flag));
        }

        reg.flip_flag(Flags::SUBTRACT);
        reg.flip_flag(Flags::CARRY);

        assert!(reg.has_flag(Flags::ZERO));
        assert!(!reg.has_flag(Flags::SUBTRACT));
        assert!(reg.has_flag(Flags::HALF_CARRY));
        assert!(!reg.has_flag(Flags::CARRY));

        reg.flip_flag(Flags::SUBTRACT);
        reg.flip_flag(Flags::CARRY);

        for flag in all_flags {
            assert!(reg.has_flag(flag));
        }
    }
}
