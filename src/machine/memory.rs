pub struct Memory {
    // the limit should be 2^16 = 65536
    data: Vec<u16>,
}

/// Bit Numbering format: right to left -> 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
impl Memory {
    pub fn new() -> Memory {
        Memory { data: vec![0; 0x10000] }
    }

    pub fn read(&self, address: u16) -> u16 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.data[address as usize] = value;
    }
}
