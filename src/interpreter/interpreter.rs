use crate::machine::{machine::{self, Machine}, registers::GeneralPurposeRegister};

use super::utils::{bit, bits_left_right, is_negative};


pub enum Instructions {
    ADD
}

pub struct Interpreter {
    instruction: Vec<u16>,
    current_idx: usize,
    machine:  Machine
}

impl Interpreter {
    pub fn new(instruction: Vec<u16>) -> Interpreter {
        Interpreter { instruction, current_idx: 0, machine:  Machine::new() }
    }

    pub fn run(self: &mut Interpreter) {
        let next_instruction = self.fetch();
        let instruction_type = self.decode_instruction_type(next_instruction);
        match instruction_type {
            Instructions::ADD => self.execute_add(next_instruction),
            _ => panic!("Instruction not supported")
        }
    }

    pub fn fetch(self: &mut Interpreter)  -> u16 {
        let instruction = self.instruction[self.current_idx];
        self.current_idx += 1;
        instruction
    }

    pub fn decode_instruction_type(&self, instruction: u16) -> Instructions {
        let i = bits_left_right(instruction, 15, 12);
        match i {
            1 => Instructions::ADD,
            _ => panic!("Instruction not supported")
        }
    }

    pub fn execute_add(&mut self,instruction: u16) {
        let machine = &mut self.machine;

        let bit_5 = bit(instruction, 5);
        match bit_5 {
            0 => {
                // register mode
                let dr = bits_left_right(instruction, 11,9);
                let sr1 = bits_left_right(instruction, 8, 6);
                let sr2 = bits_left_right(instruction, 2, 0);

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
                    match is_negative(result) {
                        true => machine.register_set.set_condition_code_negative(),
                        false => machine.register_set.set_condition_code_positive()
                    }
                }
            },
            1 => {
                                // register mode
                                let dr = bits_left_right(instruction, 11,9);
                                let sr1 = bits_left_right(instruction, 8, 6);
                                // TODO: We are missing sign extension for this value
                                let imm5 = bits_left_right(instruction, 4, 0);

                                let dr = GeneralPurposeRegister::from_u16(dr);
                                let sr1 = GeneralPurposeRegister::from_u16(sr1);

                                let sr_1_value = machine.register_set.read_general_purpose(&sr1);

                                // TODO: should this be a wrapping add? Or should we panic?
                                let result = sr_1_value.wrapping_add(imm5);

                                machine.register_set.write_to_general_purpose(&dr, result);
            },
            // todo: This panic message should be made better
            _ => panic!("Instruction not supported")
        }
        // execute the instruction
    }
}
