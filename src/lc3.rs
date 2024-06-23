use std::vec;

use crate::memory::Memory;

pub struct LC3 {
    memory: Memory,
    register_set: RegisterSet,
}

impl LC3 {
    pub fn new() -> LC3 {
        LC3 { memory: Memory::new(), register_set: RegisterSet::new() }
    }
}

pub enum GeneralPurposeRegister {
    R0, // 000
    R1, // 001
    R2, // 010
    R3, // 011
    R4, // 100
    R5, // 101
    R6, // 110
    R7 // 111
}

impl GeneralPurposeRegister {
    fn from_u16(value: u16) -> GeneralPurposeRegister {
        match value {
            0 => GeneralPurposeRegister::R0,
            1 => GeneralPurposeRegister::R1,
            2 => GeneralPurposeRegister::R2,
            3 => GeneralPurposeRegister::R3,
            4 => GeneralPurposeRegister::R4,
            5 => GeneralPurposeRegister::R5,
            6 => GeneralPurposeRegister::R6,
            7 => GeneralPurposeRegister::R7,
            _ => panic!("Invalid value for GeneralPurposeRegister")
        }
    }
}

pub struct RegisterSet {
   general_purpose: [u16; 8],
   program_counter: u16,
   // CC: N Z P, i.e CC[0] = N, CC[1] = Z, CC[2] = P
   condition_codes: [u8; 3],
   // contains the status of the current process being executed
   // PSR[15] = privelege mode, PSR[15] = 0 Supervisor, PSR[15] = 1 User
   // PSR[10:8] = priority level, PSR[2:0] = condition codes
   // PSR[2] = N, PSR[1] = Z, and PSR[0] = P.
   process_status: u16
}


impl RegisterSet {
    pub fn new() -> RegisterSet {
        RegisterSet {
            general_purpose: [0; 8],
            program_counter: 0,
            condition_codes: [0; 3],
            process_status: 0
        }
    }

    pub fn write_to_general_purpose(&mut self, register: &GeneralPurposeRegister, value: u16) {
        match register {
            GeneralPurposeRegister::R0 => self.general_purpose[0] = value,
            GeneralPurposeRegister::R1 => self.general_purpose[1] = value,
            GeneralPurposeRegister::R2 => self.general_purpose[2] = value,
            GeneralPurposeRegister::R3 => self.general_purpose[3] = value,
            GeneralPurposeRegister::R4 => self.general_purpose[4] = value,
            GeneralPurposeRegister::R5 => self.general_purpose[5] = value,
            GeneralPurposeRegister::R6 => self.general_purpose[6] = value,
            GeneralPurposeRegister::R7 => self.general_purpose[7] = value,
        }
    }

    pub fn read_general_purpose(&self, register: &GeneralPurposeRegister) -> u16 {
        match register {
            GeneralPurposeRegister::R0 => self.general_purpose[0],
            GeneralPurposeRegister::R1 => self.general_purpose[1],
            GeneralPurposeRegister::R2 => self.general_purpose[2],
            GeneralPurposeRegister::R3 => self.general_purpose[3],
            GeneralPurposeRegister::R4 => self.general_purpose[4],
            GeneralPurposeRegister::R5 => self.general_purpose[5],
            GeneralPurposeRegister::R6 => self.general_purpose[6],
            GeneralPurposeRegister::R7 => self.general_purpose[7],
        }
    }

    pub fn set_condition_code_negative(&mut self) {
        self.condition_codes[0] = 1;
        self.condition_codes[1] = 0;
        self.condition_codes[2] = 0;
    }

    pub fn set_condition_code_zero(&mut self) {
        self.condition_codes[0] = 0;
        self.condition_codes[1] = 1;
        self.condition_codes[2] = 0;
    }

    pub fn set_condition_code_positive(&mut self) {
        self.condition_codes[0] = 0;
        self.condition_codes[1] = 0;
        self.condition_codes[2] = 1;
    }
}

pub enum Instructions {
    ADD
}

pub struct Interpreter {
    instruction: Vec<u16>,
    current_idx: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { instruction: vec![], current_idx: 0 }
    }

    pub fn run(self: &mut Interpreter, machine: &mut LC3) {
        let next_instruction = self.fetch();
        let instruction_type = self.decode_instruction_type(next_instruction);
        match instruction_type {
            Instructions::ADD => self.execute_add(machine, next_instruction),
            _ => panic!("Instruction not supported")
        }
    }

    pub fn fetch(self: &mut Interpreter)  -> u16 {
        let instruction = self.instruction[self.current_idx];
        self.current_idx += 1;
        instruction
    }

    pub fn decode_instruction_type(&self, instruction: u16) -> Instructions {
        let i = instructions_utils::bits_left_right(instruction, 15, 12);
        match i {
            1 => Instructions::ADD,
            _ => panic!("Instruction not supported")
        }
    }

    pub fn execute_add(&self, machine: &mut LC3, instruction: u16) {
        let bit_5 = instructions_utils::bit(instruction, 5);
        match bit_5 {
            0 => {
                // register mode
                let dr = instructions_utils::bits_left_right(instruction, 11,9);
                let sr1 = instructions_utils::bits_left_right(instruction, 8, 6);
                let sr2 = instructions_utils::bits_left_right(instruction, 2, 0);

                let dr = GeneralPurposeRegister::from_u16(dr);
                let sr1 = GeneralPurposeRegister::from_u16(sr1);
                let sr2 = GeneralPurposeRegister::from_u16(sr2);

                let sr1_value = machine.register_set.read_general_purpose(&sr1);
                let sr2_value = machine.register_set.read_general_purpose(&sr2);

                let result = sr1_value.wrapping_add(sr2_value);
                machine.register_set.write_to_general_purpose(&dr, result);

                if result == 0
                {
                    machine.register_set.set_condition_code_zero();
                }
                else {
                    match instructions_utils::is_negative(result) {
                        true => machine.register_set.set_condition_code_negative(),
                        false => machine.register_set.set_condition_code_positive()
                    }
                }
            },
            1 => {
                panic!("Immediate Mode not supported");
            },
            // todo: This panic message should be made better
            _ => panic!("Instruction not supported")
        }
        // execute the instruction
    }
}

mod instructions_utils {
    // get the range of bits from left[inclusive] to right [inclusive]
    // ex: bits_left_right(0b1010, 3, 1) -> 0b101
    pub fn bits_left_right(instruction: u16, left: u16, right: u16) -> u16 {
        let range = (left - right) + 1;
        let shifted_value = instruction >> right;

        let mask = (1 << range ) - 1;

        shifted_value & mask
    }

    // get the bit value at a specific index
    pub fn bit(instruction: u16, idx: u16) -> u16 {
        (instruction >> idx) & 0x1
    }

    // is the value negative in 2s complement form
    pub fn is_negative(value: u16) -> bool{
        let value =  self::bit(value, 15);
        match value {
            0 => false,
            1 => true,
            _ => panic!("Invalid value for bit")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lc3::LC3;

    #[test]
    fn test_add(){
        let mut machine = LC3::new();

        machine.register_set.write_to_general_purpose(&GeneralPurposeRegister::R1, 11);
        machine.register_set.write_to_general_purpose(&GeneralPurposeRegister::R2, 5);
        let mut interpreter = Interpreter::new();
        interpreter.instruction = vec![0b0001_000_001_0_00_010];

        interpreter.run(&mut machine);

        println!("R0: {}", machine.register_set.read_general_purpose(&GeneralPurposeRegister::R0));
    }
}
