#![cfg(test)]

use super::*;

extern crate quickcheck;

#[quickcheck]
fn encode_is_inverse_of_decode(input: Vec<u8>) -> bool {
    decode(&encode(input.as_slice()).join(" ")) == Ok(input)
}
