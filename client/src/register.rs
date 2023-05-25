use std::{io, str::FromStr};

use num_bigint::{BigInt, ToBigInt, Sign};

use crate::shared::{P, G, H, mod_exp};

// The user provides email and password via standard input, and y1 and y2 are generated afterwards using the formula.
pub fn get_register_data() -> (String, BigInt, String, String) {
    println!("registering");

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

    let x = BigInt::from_bytes_le(Sign::Plus, &password.as_bytes());

    // y1 = g ^ x % p
    // y2 = h ^ x % p
    let y1 = mod_exp(&G.to_bigint().unwrap(), &x, &BigInt::from_str(P).unwrap()).to_string();
    let y2 = mod_exp(&H.to_bigint().unwrap(), &x, &BigInt::from_str(P).unwrap()).to_string();

    return (email, x, y1, y2)
}