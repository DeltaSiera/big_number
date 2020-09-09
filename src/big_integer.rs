use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Add, Mul, Shl, Not, BitAnd, Shr};
use itertools::EitherOrBoth::{Left, Both, Right};
use itertools::{Itertools};


//each item of vector is 32bit unsigned word
pub struct BigInt {
    number: Vec<u32>,
}

impl BigInt {
    pub fn new(input_number: &str) -> Self {
        let changed_number = BigInt::change(input_number);

        BigInt {
            number: changed_number,
        }
    }

    fn change(number: &str) -> Vec<u32> {
        let number_len = number.len();
        let mut cursor = 0;
        let number_digits = number_len;
        let number_bits = ((number_digits * 3402) >> 10) + 1;
        let num_words = (number_bits + 31) >> 5;
        let mut magnitude: Vec<u32> = vec![0; num_words];
        let mut first_group = number_digits % 9;
        if first_group == 0 {
            first_group = 9;
        }

        let mut group = &number[cursor..first_group];
        cursor += first_group;
        magnitude[num_words - 1] = u32::from_str_radix(group, 10).unwrap();
        while cursor < number_len {
            group = &number[cursor..cursor + 9];
            cursor += 9;
            let group_value = u32::from_str_radix(group, 10).unwrap();
            BigInt::change_number_base(&mut magnitude, group_value);
        }
        if magnitude[0] == 0 {
            magnitude.remove(0);
        }
        magnitude
    }

    fn change_number_base(number: &mut Vec<u32>, z: u32) {
        let mut carry = 0;

        for x in number.iter_mut().rev() {
            let product = (1000000000 * (*x as u64)) + carry;
            *x = product as u32;
            carry = product >> 32;
        }

        carry = 0u64 + z as u64;
        for y in number.iter_mut().rev() {
            let sum = ((*y as u64) + carry) as u64;
            *y = sum as u32;                            //same as &0xFFFF_FFFF
            carry = sum >> 32;
        }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for x in self.number.iter() {
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

impl<'a> Add for &'a BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        let mut number = vec![];
        let mut carry = 0;
        for tuple in self.number.iter().rev().zip_longest(rhs.number.iter().rev()) {
            let sum = match tuple {
                Both(&x, &y) => x as u64 + y as u64 + carry,
                Left(&x) => x as u64 + carry,
                Right(&y) => y as u64 + carry,
            };
            number.push((sum & 0xFFFF_FFFF) as u32);
            carry = (sum >> 32) as u64;
        }

        if carry != 0 {
            number.push(carry as u32);
        }
        BigInt {
            number,
        }
    }
}

impl<'a> BitAnd<&'a BigInt> for &'a BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: &'a BigInt) -> Self::Output {
        let and_op_result = self.number.iter()
                                .rev()
                                .zip(rhs.number.iter().rev())
                                .map(|(&x, &y)| x & y)
                                .rev()
                                .collect();

        BigInt { number: and_op_result, }
    }
}

impl Not for BigInt {
    type Output = BigInt;

    fn not(self) -> Self::Output {
        let not_op_result = self.number.iter()
                                .map(|&n| !n)
                                .collect();
        BigInt {
            number: not_op_result,
        }
    }
}

//implement later
impl<'a> Mul for &'a BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut mul_op_result: Vec<_> = vec![0; self.number.len() + rhs.number.len()];
        mul_op_result.push(1);
        BigInt {
            number: mul_op_result,
        }
    }
}

//takes bytes from left to right
impl<'a> Shl<u32> for &'a BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u32) -> Self::Output {
        let words_increment = (rhs >> 5) as usize;
        let shift_left = rhs & 0x1f;
        let current_number_len = self.number.len();
        let mut shift_left_op_result = vec![0; current_number_len + words_increment];
        let shift_right = (32 - shift_left);
        let highest_bits = self.number[0].checked_shr(shift_right).unwrap_or(0);
        let mut counter = 0;
        if highest_bits != 0 {
            shift_left_op_result.push(0);               //additional zero
            shift_left_op_result[counter] = (highest_bits);
            counter += 1;
        }

        let mut result_tuple_iterator = self.number.iter()
                                            .tuple_windows();

        for (current, right) in result_tuple_iterator {
            println!("{}|{}", right, current);
            let shifted_left = current << (shift_left);
            let shifted_right = right.checked_shr(shift_right).unwrap_or(0);
            shift_left_op_result[counter] = shifted_left | shifted_right;
            counter += 1;
        }
        shift_left_op_result[counter] = self.number.last()
                                            .unwrap()
                                            .checked_shl(shift_left)
                                            .unwrap_or(0);                  //last element to needs to be shifted after loops

        BigInt {
            number: shift_left_op_result,
        }
    }
}

impl<'a> Shr<u8> for &'a BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u8) -> Self::Output {
        // let mut shift_right_op_result = vec![];
        let shift_right = rhs & 0x1f;

        BigInt {
            number: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_zero() {
        let number_zero = BigInt::new("0");
        assert_eq!("0", "0");
    }

    #[test]
    fn one() {
        let number_one = BigInt::new("1");
        assert_eq!("1", "1");
    }

    #[test]
    fn add() {}
}