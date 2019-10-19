use super::parity::Parity;
use super::parity::SwappingParity;
use super::wordlists;

pub fn for_byte(byte: u8, parity: Parity) -> &'static str {
    match parity {
        Parity::Even => wordlists::EVEN[byte as usize],
        Parity::Odd => wordlists::ODD[byte as usize],
    }
}

/// Encode a vector of bytes using the PGP Word List.
///
/// ```rust
/// # use mjecrypto::encoding::pgp::encode;
/// let test_bytes = [
///     0xE5, 0x82, 0x94, 0xF2,
///     0xE9, 0xA2, 0x27, 0x48,
///     0x6E, 0x8B, 0x06, 0x1B,
///     0x31, 0xCC, 0x52, 0x8F,
///     0xD7, 0xFA, 0x3F, 0x19,
/// ];
/// let expected_string = "topmost Istanbul Pluto vagabond treadmill Pacific brackish dictator goldfish Medusa afflict bravado chatter revolver Dupont midsummer stopwatch whimsical cowbell bottomless";
/// assert_eq!(
///     encode(&test_bytes).join(" "),
///     expected_string
/// );
/// ```
pub fn encode(input: &[u8]) -> Vec<&str> {
    let mut parity: SwappingParity = SwappingParity::new(Parity::Even);
    input
        .iter()
        .map(|byte| for_byte(*byte, parity.current_value_with_swap_side_effect()))
        .collect::<Vec<&str>>()
}
