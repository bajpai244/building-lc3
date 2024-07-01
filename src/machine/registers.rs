pub enum GeneralPurposeRegister {
    R0, // 000
    R1, // 001
    R2, // 010
    R3, // 011
    R4, // 100
    R5, // 101
    R6, // 110
    R7, // 111
}

impl GeneralPurposeRegister {
    pub fn from_u16(value: u16) -> GeneralPurposeRegister {
        match value {
            0 => GeneralPurposeRegister::R0,
            1 => GeneralPurposeRegister::R1,
            2 => GeneralPurposeRegister::R2,
            3 => GeneralPurposeRegister::R3,
            4 => GeneralPurposeRegister::R4,
            5 => GeneralPurposeRegister::R5,
            6 => GeneralPurposeRegister::R6,
            7 => GeneralPurposeRegister::R7,
            _ => panic!("Invalid value for GeneralPurposeRegister"),
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
    process_status: u16,
}

impl RegisterSet {
    pub fn new() -> RegisterSet {
        RegisterSet {
            general_purpose: [0; 8],
            program_counter: 0,
            condition_codes: [0; 3],
            process_status: 0,
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
