#![allow(unused)]
use std::ops::Sub;
use std::str::FromStr;

use num_bigint::{BigInt, ToBigInt, RandBigInt, Sign};
use num_traits::{Zero, One, FromPrimitive};

pub const P: &str = "23"; // this should be a large random prime number, 2048b
pub const Q: &str = "11"; // (P - 1) / 2
pub const G: i32 = 4;
pub const H: i32 = 9;

pub const BIT_SIZE: u64 = 64;

pub fn generate_random() -> BigInt {
    return rand::thread_rng().gen_bigint(BIT_SIZE);
}

pub fn generate_random_in_range(start: &BigInt, end: &BigInt) -> BigInt {
    return rand::thread_rng().gen_bigint_range(start, end);
}


// Used this implementation https://docs.rs/mod_exp/latest/src/mod_exp/lib.rs.html#1-90 with modification for big int
pub fn mod_exp(base: &BigInt, exponent: &BigInt, modulus: &BigInt) -> BigInt {
    let one: BigInt = One::one();
    let two: BigInt = &one + &one;
    let zero: BigInt = Zero::zero();

    let mut result = 1.to_bigint().unwrap();
    let mut base = base % modulus;
    let mut exponent = exponent.clone();

    loop {

        if exponent <= zero {
            break;
        }

        if &exponent % &two == one {
            result = (result * &base) % modulus;
        }

        exponent = exponent >> 1;
        base = &base * &base % modulus;
    }

    return result

}


#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_traits::{Zero, One, FromPrimitive};

    use std::ops::Sub;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn verify_mod_exp() {
        let base = BigInt::from_i32(4).unwrap();
        let exp = BigInt::from_i32(6).unwrap();
        let modulus = BigInt::from_i32(23).unwrap();
       

        let res = mod_exp(&base, &exp, &modulus);
        assert_eq!(res.to_string(), "2");
    }
}