use super::memory::Memory;
use super::registers::RegisterSet;

// LC3 Machine
pub struct Machine {
    pub memory: Memory,
    pub register_set: RegisterSet,
}

impl Machine {
    pub fn new() -> Machine {
        Machine { memory: Memory::new(), register_set: RegisterSet::new() }
    }
}
