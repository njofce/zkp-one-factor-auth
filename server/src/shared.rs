#![allow(unused)]
use std::ops::Sub;
use std::str::FromStr;

use num_bigint::{BigInt, ToBigInt, RandBigInt, Sign};
use num_traits::{Zero, One, FromPrimitive};

pub const P: &str = "23"; // this should be a large random prime number, ex. 2048b
pub const Q: &str = "11"; // (P - 1) / 2
pub const G: i32 = 4;
pub const H: i32 = 9;

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


// Run the verification formula
pub fn verify(y1: BigInt, y2: BigInt, r1: BigInt, r2: BigInt, c: BigInt, s: BigInt) -> bool {
    let p = BigInt::from_str(P).unwrap();

    // (((G^s %p) * (y1 ^ c %p) % p) + p) % p
    // (((H^s %p) * (y2 ^ c %p) % p) + p) % p
    let result1 = ((mod_exp(&G.to_bigint().unwrap(), &s, &p ) * mod_exp(&y1, &c, &p) % &p) + &p) % &p;
    let result2 = ((mod_exp(&H.to_bigint().unwrap(), &s, &p) * mod_exp(&y2, &c, &p) % &p) + &p) % &p;

    return &r1 == &result1 && &r2 == &result2;
}


#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_traits::{Zero, One, FromPrimitive};

    use std::ops::Sub;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn verify_test() {
        let r1 = BigInt::from_i32(13).unwrap();
        let r2 = BigInt::from_i32(2).unwrap();
        let c = BigInt::from_i32(3).unwrap();
        let s = BigInt::from_i32(7).unwrap();
        let y1 = BigInt::from_i32(9).unwrap();
        let y2 = BigInt::from_i32(13).unwrap();

        let res = verify(y1, y2, r1, r2, c, s);
        assert_eq!(res, true);
    }
}