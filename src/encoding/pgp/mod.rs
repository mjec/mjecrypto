mod decode;
mod encode;
mod parity;
mod tests;
pub mod wordlists;

pub use decode::{decode, DecodeError};
pub use encode::encode;
