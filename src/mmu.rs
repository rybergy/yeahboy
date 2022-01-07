pub struct Mmu {
    /// Cartridge RAM
    cart: Vec<u8>,

    /// Video RAM
    vram: Vec<u8>,

    /// Cartridge (external) RAM
    cart_ram: Vec<u8>,

    /// Working (internal) RAM
    ram: Vec<u8>,

    /// Object attribute memory
    oam: Vec<u8>,

    /// Zero-page RAM
    zpram: Vec<u8>,
}

impl Mmu {
    pub fn new(cart: Vec<u8>) -> Self {
        let mut mmu = Self {
            cart,
            vram: vec![0; 0x2000],
            cart_ram: vec![0; 0x2000],
            ram: vec![0; 0x2000],
            oam: vec![0; 0x100],
            zpram: vec![0; 0x100],
        };

        mmu.wb(0xFF05, 0x00);
        mmu.wb(0xFF06, 0x00);
        mmu.wb(0xFF07, 0x00);
        mmu.wb(0xFF10, 0x80);
        mmu.wb(0xFF11, 0xBF);
        mmu.wb(0xFF12, 0xF3);
        mmu.wb(0xFF14, 0xBF);
        mmu.wb(0xFF16, 0x3F);
        mmu.wb(0xFF17, 0x00);
        mmu.wb(0xFF19, 0xBF);
        mmu.wb(0xFF1A, 0x7F);
        mmu.wb(0xFF1B, 0xFF);
        mmu.wb(0xFF1C, 0x9F);
        mmu.wb(0xFF1E, 0xBF);
        mmu.wb(0xFF20, 0xFF);
        mmu.wb(0xFF21, 0x00);
        mmu.wb(0xFF22, 0x00);
        mmu.wb(0xFF23, 0xBF);
        mmu.wb(0xFF24, 0x77);
        mmu.wb(0xFF25, 0xF3);
        mmu.wb(0xFF26, 0xF1);
        mmu.wb(0xFF40, 0x91);
        mmu.wb(0xFF42, 0x00);
        mmu.wb(0xFF43, 0x00);
        mmu.wb(0xFF45, 0x00);
        mmu.wb(0xFF47, 0xFC);
        mmu.wb(0xFF48, 0xFF);
        mmu.wb(0xFF49, 0xFF);
        mmu.wb(0xFF4A, 0x00);
        mmu.wb(0xFF4B, 0x00);
        mmu.wb(0xFFFF, 0x00);

        mmu
    }

    // pub fn raw_memory(&self) -> &Vec<u8> {
    //     &self.mem
    // }

    pub fn wb(&mut self, addr: usize, value: u8) {
        match addr {
            // 0x0000-0x8000: Cartridge memory. For MBC this will need to handle indexing
            //                further into the cart
            0x0000..=0x7FFF => self.cart[addr] = value, // TODO: ROM bank switching
            // 0x8000-0xA000: Video RAM
            0x8000..=0x9FFF => self.vram[addr - 0x8000] = value,
            // 0xA000-0xC000: Cartridge (external) RAM
            0xA000..=0xBFFF => self.cart_ram[addr - 0xA000] = value, // TODO: RAM bank switching
            // 0xC000-0xE000: Working (internal) RAM
            0xC000..=0xDFFF => self.ram[addr - 0xC000] = value,
            // 0xE000-0xFE00: Shadow of working rAM
            0xE000..=0xFDFF => self.ram[addr - 0xE000] = value,
            // 0xFE00-0xFEA0: Object attrbute memory
            0xFE00..=0xFE9F => self.oam[addr - 0xFE00] = value,
            // 0xFEA0-0xFF00: All zeroes
            0xFEA0..=0xFEFF => (),
            // 0xFF00-0x10000: Zero-page RAM
            0xFF00..=0xFFFF => self.zpram[addr - 0xFF00] = value,
            _ => todo!(),
        }
    }

    pub fn ww(&mut self, addr: usize, value: u16) {
        self.wb(addr, value as u8);
        self.wb(addr + 1, (value >> 8) as u8);
    }

    pub fn rb(&self, addr: usize) -> u8 {
        match addr {
            // 0x0000-0x8000: Cartridge memory. For MBC this will need to handle indexing
            //                further into the cart
            0x0000..=0x7FFF => self.cart[addr], // TODO: ROM bank switching
            // 0x8000-0xA000: Video RAM
            0x8000..=0x9FFF => self.vram[addr - 0x8000],
            // 0xA000-0xC000: Cartridge (external) RAM
            0xA000..=0xBFFF => self.cart_ram[addr - 0xA000], // TODO: RAM bank switching
            // 0xC000-0xE000: Working (internal) RAM
            0xC000..=0xDFFF => self.ram[addr - 0xC000],
            // 0xE000-0xFE00: Shadow of working rAM
            0xE000..=0xFDFF => self.ram[addr - 0xE000],
            // 0xFE00-0xFEA0: Object attrbute memory
            0xFE00..=0xFE9F => self.oam[addr - 0xFE00],
            // 0xFEA0-0xFF00: All zeroes
            0xFEA0..=0xFEFF => 0,
            // 0xFF00-0x10000: Zero-page RAM
            0xFF00..=0xFFFF => self.zpram[addr - 0xFF00],
            _ => todo!(),
        }
    }

    pub fn rw(&self, addr: usize) -> u16 {
        ((self.rb(addr + 1) as u16) << 8) | (self.rb(addr) as u16)
    }
}

#[cfg(test)]
mod test {
    use super::Mmu;

    #[test]
    fn read_write_bytes() {
        let mut mmu = Mmu::new(vec![0; 0x8000]);

        let addr = 0x1234;

        assert!(mmu.rb(addr) == 0);

        mmu.wb(addr, 0x67);
        assert!(mmu.rb(addr) == 0x67);

        mmu.wb(addr + 1, 0x79);
        assert!(mmu.rb(addr + 1) == 0x79);
        assert!(mmu.rw(addr) == 0x7967);

        mmu.ww(addr, 0x5248);
        assert!(mmu.rb(addr) == 0x48);
        assert!(mmu.rb(addr + 1) == 0x52);
        assert!(mmu.rw(addr) == 0x5248);
    }
}
