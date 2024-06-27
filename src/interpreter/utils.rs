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
