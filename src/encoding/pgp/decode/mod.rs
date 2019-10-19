extern crate phf;

mod error;

use super::parity::{Parity, SwappingParity};
use super::wordlists;
pub use error::DecodeError;

/// Decode a string of PGP Word List words into bytes.
///
/// ```rust
/// # use mjecrypto::encoding::pgp::decode;
/// let test_string = "topmost Istanbul Pluto vagabond treadmill Pacific brackish dictator goldfish Medusa afflict bravado chatter revolver Dupont midsummer stopwatch whimsical cowbell bottomless";
/// let expected_bytes: Vec<u8> = vec![
///     0xE5, 0x82, 0x94, 0xF2,
///     0xE9, 0xA2, 0x27, 0x48,
///     0x6E, 0x8B, 0x06, 0x1B,
///     0x31, 0xCC, 0x52, 0x8F,
///     0xD7, 0xFA, 0x3F, 0x19,
/// ];
/// assert_eq!(
///     decode(test_string),
///     Ok(expected_bytes)
/// );
/// ```
pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
    let mut parity: SwappingParity = SwappingParity::new(Parity::Even);
    let mut result: Vec<u8> = Vec::new();
    for (position, word) in input.split_whitespace().enumerate() {
        match from_word(word, parity.current_value_with_swap_side_effect()) {
            Some(byte) => result.push(byte),
            _ => match from_word(word, parity.parity) {
                Some(byte_value) => {
                    return Err(DecodeError::ParityError {
                        position,
                        word: word.to_string(),
                        actual_parity: parity,
                        byte_value,
                    })
                }
                _ => {
                    return Err(DecodeError::UnrecognizedWord {
                        position,
                        word: word.to_string(),
                    })
                }
            },
        }
    }
    Ok(result)
}

pub fn from_word(word: &str, parity: Parity) -> Option<u8> {
    match parity {
        Parity::Even => &wordlists::EVEN_LOOKUP_LOWERCASE,
        Parity::Odd => &wordlists::ODD_LOOKUP_LOWERCASE,
    }
    .get(word.to_lowercase().as_str())
    .copied()
}
