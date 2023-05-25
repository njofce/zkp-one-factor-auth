use std::{io, str::FromStr};
use std::ops::Sub;
use num_bigint::{BigInt, ToBigInt, Sign};
use num_traits::{FromPrimitive};

use crate::shared::{G, H, P, Q, mod_exp, generate_random_in_range};

// The user needs to enter their email and password via standard input, then the password is converted to a number based on its bytes (this is X), and then a random K is created which is used to generate Y1 and Y2
pub fn get_login_data() -> (String, BigInt, BigInt, String, String) {
    println!("logging in");

    let mut email = String::new();
    let mut password = String::new();

    println!("enter your email");
    io::stdin()
        .read_line(&mut email)
        .expect("Error");

    println!("enter your password");

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password");

    // Convert password to BigInt using bytes in little endian
    let x = BigInt::from_bytes_le(Sign::Plus, &password.as_bytes());

    // Generate random K
    let k = generate_random_in_range(&BigInt::from_i32(2).unwrap(), &BigInt::from_str(&P).unwrap().sub(2)); // K would be in range (2, p-2)

    // Generate r1 & r2
    // r1 = g^k % p
    // r2 = h^k % p
    let r1 = mod_exp(&G.to_bigint().unwrap(), &k, &BigInt::from_str(P).unwrap()).to_string();
    let r2 =  mod_exp(&H.to_bigint().unwrap(), &k, &BigInt::from_str(P).unwrap()).to_string();

    return (email, x, k, r1, r2);
}

// Compute S based on the challange
// S= (((k-c*x) % q + q) % q)
pub fn compute_s(x: BigInt, c: BigInt, k: BigInt) -> BigInt {
    return (((&k - &c * &x) % (&BigInt::from_str(&Q).unwrap())) + (&BigInt::from_str(&Q).unwrap())) % (&BigInt::from_str(&Q).unwrap());
}