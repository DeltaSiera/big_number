use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Add, Mul, Shl, Not, BitAnd, Shr, AddAssign};
use itertools::EitherOrBoth::{Left, Both, Right};
use itertools::{Itertools};
use num_bigint::BigUint;

const NUMBER_BIT_LENGTH: u32 = 32;


//each item of vector is 32bit unsigned word
pub struct BigInt {
    pub number: Vec<u32>,
}

impl BigInt {
    pub fn new(input_number: &str) -> Self {
        let changed_number = BigInt::change_number_base(input_number);

        BigInt {
            number: changed_number,
        }
    }
    pub fn from_number(number: u32) -> BigInt {
        BigInt {
            number: vec![number],
        }
    }
    fn change_number_base(number: &str) -> Vec<u32> {
        let number_len = number.len();
        let mut cursor = 0;
        let number_digits = number_len;
        let number_bits = ((number_digits * 3402) >> 10) + 1;
        let num_words = (number_bits + 31) >> 5;
        let mut number_vec: Vec<u32> = vec![0; num_words];
        let mut first_group = number_digits % 9;
        if first_group == 0 {
            first_group = 9;
        }

        let mut group = &number[cursor..first_group];
        cursor += first_group;
        // magnitude[num_words - 1] = u32::from_str_radix(group, 10).unwrap();                    //if number  should be small endian
        number_vec[0] = u32::from_str_radix(group, 10).unwrap();                          //big endian
        while cursor < number_len {
            group = &number[cursor..cursor + 9];
            cursor += 9;
            let group_value = u32::from_str_radix(group, 10).unwrap();
            BigInt::perform_multiplication(&mut number_vec, group_value);
        }
        if *number_vec.last().unwrap() == 0 {
            number_vec.truncate(number_vec.len() - 1);
        }
        number_vec
    }

    fn perform_multiplication(number: &mut Vec<u32>, z: u32) {
        let mut carry = 0;

        for x in number.iter_mut() {
            let product = (1000000000 * (*x as u64)) + carry;
            *x = product as u32;
            carry = product >> 32;
        }

        carry = 0u64 + z as u64;
        for y in number.iter_mut() {
            let sum = ((*y as u64) + carry) as u64;
            *y = sum as u32;                            //same as &0xFFFF_FFFF
            carry = sum >> 32;
        }
    }
    fn all_zero(number: &Vec<u32>) -> bool {
        number.iter().all(|&x| x == 0)
    }
    fn divide_by10(number: &mut Vec<u32>) -> u8 {
        let mut reminder = 0u64;
        for x in number.iter_mut().rev() {
            let d = (*x as u64 + (reminder << 32)) / 10;
            reminder = (*x as u64 + (reminder << 32)) % 10;
            *x = d as u32;
        }
        reminder as u8
    }
    fn convert_to_decimal(&self) -> Vec<u8> {
        let mut result = vec![];
        let mut copy_self = self.number.clone();
        // if self.number.len() == 1 && self.number.first().unwrap() == 0 {
        //     return vec![0];
        // }
        while !BigInt::all_zero(&copy_self) {
            result.push(BigInt::divide_by10(&mut copy_self));
        }
        result
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // let result = BigUint::new(self.number.clone());
        for x in self.convert_to_decimal().iter().rev() {
            write!(f, "{}", x)?;
        }
        Ok(())
    }
}

//multiplication using russian peasant method
impl<'a> Mul<&'a BigInt> for &'a BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &'a BigInt) -> Self::Output {
        let mut mul_op_result = BigInt::new("0");
        let one = BigInt::new("1");
        let mut copy_self = self.clone();
        let mut copy_rhs = rhs.clone();
        let mut x = BigInt::from_number(0);
        let mut y = BigInt::from_number(0);
        while !copy_rhs.number.is_empty() {
            if (copy_rhs & &one).number[0] == 1 {
                mul_op_result = &mul_op_result + copy_self;
            }
            x = copy_self << 1;
            y = copy_rhs >> 1;
            copy_self = &x;
            copy_rhs = &y;
        }
        mul_op_result
    }
}

impl<'a> Add<&'a BigInt> for &'a BigInt {
    type Output = BigInt;

    fn add(self, other: &BigInt) -> BigInt {
        // self += other;
        // self
        let mut number = vec![];
        let mut carry = 0;
        for tuple in self.number.iter().zip_longest(other.number.iter()) {
            let sum = match tuple {
                Both(&x, &y) => x as u64 + y as u64 + carry,
                Left(&x) => x as u64 + carry,
                Right(&y) => y as u64 + carry,
            };
            number.push((sum & 0xFFFF_FFFF) as u32);
            carry = (sum >> 32) as u64;
        }

        while carry != 0 {
            let x = carry & 0xFFFF_FFFF;
            number.push(x as u32);
            carry >>= 32;
        }
        BigInt {
            number
        }
    }
}

//re-implement
impl<'a> AddAssign<&'a BigInt> for BigInt {
    fn add_assign(&mut self, rhs: &BigInt) {
        // let mut number = vec![];
        // let mut carry = 0;
        // for tuple in self.number.iter().rev().zip_longest(rhs.number.iter().rev()) {
        //     let sum = match tuple {
        //         Both(&x, &y) => x as u64 + y as u64 + carry,
        //         Left(&x) => x as u64 + carry,
        //         Right(&y) => y as u64 + carry,
        //     };
        //     number.push((sum & 0xFFFF_FFFF) as u32);
        //     carry = (sum >> 32) as u64;
        // }
        //
        // if carry != 0 {
        //     number.push(carry as u32);
        // }
        // number.reverse();
    }
}

impl<'a> BitAnd<&'a BigInt> for &'a BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: &'a BigInt) -> Self::Output {
        let and_op_result = self.number.iter()
                                .zip(rhs.number.iter().rev())
                                .map(|(&x, &y)| x & y)
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


//takes bytes from left to right
impl<'a> Shl<u32> for &'a BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u32) -> Self::Output {
        let words_increment = (rhs >> 5) as usize;
        let shift_left = rhs & 0x1f;
        let current_number_len = self.number.len();
        let mut shift_left_op_result = vec![0; current_number_len + words_increment];
        let shift_right = NUMBER_BIT_LENGTH - shift_left;

        let highest_bits = self.number.last()
                               .unwrap()
                               .checked_shr(shift_right)
                               .unwrap_or(0);

        let mut cursor = current_number_len + words_increment;
        if highest_bits != 0 {
            cursor += 1;
            shift_left_op_result.push(0);               //additional zero
            shift_left_op_result[cursor - 1] = highest_bits;
            cursor -= 1;
        }


        let result_tuple_iterator = self.number.iter()
                                        .rev()
                                        .tuple_windows();

        for (current, right) in result_tuple_iterator {
            let shifted_left = current << shift_left;
            let shifted_right = right.checked_shr(shift_right)
                                     .unwrap_or(0);
            shift_left_op_result[cursor - 1] = shifted_left | shifted_right;
            cursor -= 1;
        }

        shift_left_op_result[cursor - 1] = self.number.first()
                                               .unwrap()
                                               .checked_shl(shift_left)
                                               .unwrap_or(0);                  //last non zero element to needs to be shifted after loops

        BigInt {
            number: shift_left_op_result,
        }
    }
}
//needs to analyze code and refactor
impl<'a> Shr<u32> for &'a BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut copy_self = self.number.clone();
        copy_self.reverse();
        let n_ints = rhs >> 5;
        let n_bits = rhs & 0x1f;
        let number_length = self.number.len();
        let mut result = vec![];
        if n_ints >= number_length as u32 {
            return BigInt {
                number: vec![0],
            }
        }
        if n_bits == 0 {
            return BigInt {
                number: copy_self[number_length - n_ints as usize..].to_owned(),
            }
        } else {
            let mut i = 0;
            let high_bits = copy_self[0] >> n_bits;
            if high_bits != 0 {
                result = vec![0; number_length - n_ints as usize];
                result[i] = high_bits;
                i += 1;
            } else {
                result = vec![0; number_length - n_ints as usize - 1];
            }
            let n_bits2 = NUMBER_BIT_LENGTH - n_bits;
            let mut j = 0;
            while j < number_length - n_ints as usize - 1 {
                result[i] = copy_self[j] << n_bits2 | copy_self[j + 1] >> n_bits;
                i += 1;
                j += 1;
            }
        }
        result.reverse();
        BigInt {
            number: result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

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
