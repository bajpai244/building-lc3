use core::num;

// get the range of bits from left[inclusive] to right [inclusive]
// ex: bits_left_right(0b1010, 3, 1) -> 0b101
pub fn bits_left_right(instruction: u16, left: u16, right: u16) -> u16 {
    let range = (left - right) + 1;
    let shifted_value = instruction >> right;

    let mask = (1 << range) - 1;

    shifted_value & mask
}

// get the bit value at a specific index
pub fn bit(instruction: u16, idx: u16) -> u16 {
    (instruction >> idx) & 0x1
}

// is the value negative in 2s complement form
pub fn is_negative(value: u16) -> bool {
    let value = self::bit(value, 15);
    match value {
        0 => false,
        1 => true,
        _ => panic!("Invalid value for bit"),
    }
}

/// get the number of bits required to represent the number
pub fn num_of_bits(num: u16) -> u16 {
    if num == 0 {
        return 1;
    }

    let mut count = 0;
    let mut num = num;

    while num > 0 {
        num = num >> 1;
        count += 1;
    }

    count
}

/// sign extend the number
/// The most significant bit of `num` is replicated as many times as necessary to
// extend A to 16 bits. For example, if `num` = 110000, then SEXT(`num`) = 1111 1111
// 1111 0000.
pub fn sign_extend(num: u16) -> u16 {
    if (num == 0) {
        return 0;
    }

    let num_of_bits = self::num_of_bits(num);
    let mask = 0xFFFF << num_of_bits;

    return num + mask;
}

#[cfg(test)]
mod test {
    use super::sign_extend;

    #[test]
    fn test_sign_extend_most_significant_bit_is_1() {
        let num: u16 = 0b110000;
        let result = sign_extend(num);
        assert_eq!(result, 0b1111_1111_1111_0000);

        let num: u16 = 0b110001;
        let result = sign_extend(num);
        assert_eq!(result, 0b1111_1111_1111_0001);

        let num: u16 = 0b110101;
        let result = sign_extend(num);
        assert_eq!(result, 0b1111_1111_1111_0101);

        let num: u16 = 0b1;
        let result = sign_extend(num);
        assert_eq!(result, 0b1111_1111_1111_1111);
    }

    #[test]
    fn test_sign_extend_0() {
        let num: u16 = 0;
        let result = sign_extend(num);
        assert_eq!(result, 0);
    }
}
