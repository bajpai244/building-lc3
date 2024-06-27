// use std::vec;

// use crate::memory::Memory;

// pub struct LC3 {
//     memory: Memory,
//     register_set: RegisterSet,
// }

// impl LC3 {
//     pub fn new() -> LC3 {
//         LC3 { memory: Memory::new(), register_set: RegisterSet::new() }
//     }
// }

// mod instructions_utils {

// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::lc3::LC3;

//     #[test]
//     fn test_add(){
//         let mut machine = LC3::new();

//         machine.register_set.write_to_general_purpose(&GeneralPurposeRegister::R1, 11);
//         machine.register_set.write_to_general_purpose(&GeneralPurposeRegister::R2, 5);
//         let mut interpreter = Interpreter::new();
//         interpreter.instruction = vec![0b0001_000_001_0_00_010];

//         interpreter.run(&mut machine);

//         println!("R0: {}", machine.register_set.read_general_purpose(&GeneralPurposeRegister::R0));
//     }
// }
