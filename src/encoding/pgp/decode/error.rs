use super::SwappingParity;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum DecodeError {
    ParityError {
        position: usize,
        word: String,
        actual_parity: SwappingParity,
        byte_value: u8,
    },
    UnrecognizedWord {
        position: usize,
        word: String,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParityError { word, actual_parity, byte_value, .. } => write!(
                f,
                "Parity error: the {} word ('{}', decodes to 0x{:x}) matches {} parity, but should be {}",
                self.position_as_ordinal_string(),
                word,
                byte_value,
                actual_parity.parity,
                actual_parity.other(),
            ),
            Self::UnrecognizedWord { word, .. } => write!(
                f,
                "Unrecognized word '{}' (occurs as {} word)",
                word,
                self.position_as_ordinal_string()
            ),
        }
    }
}

impl DecodeError {
    fn position_as_ordinal_string(&self) -> String {
        let ordinal = match self {
            Self::ParityError { position, .. } => position + 1,
            Self::UnrecognizedWord { position, .. } => position + 1,
        };
        format!(
            "{}{}",
            ordinal,
            match ordinal % 10 {
                1 if ordinal % 100 != 11 => "st",
                2 if ordinal % 100 != 12 => "nd",
                3 if ordinal % 100 != 13 => "rd",
                _ => "th",
            }
        )
    }
}

#[cfg(test)]
#[test]
fn test_decode_error_ordinals() {
    assert_ordinal_suffix(1, "st");
    assert_ordinal_suffix(2, "nd");
    assert_ordinal_suffix(3, "rd");
    assert_ordinal_suffix(4, "th");
    assert_ordinal_suffix(10, "th");
    assert_ordinal_suffix(11, "th");
    assert_ordinal_suffix(12, "th");
    assert_ordinal_suffix(13, "th");
    assert_ordinal_suffix(14, "th");
    assert_ordinal_suffix(20, "th");
    assert_ordinal_suffix(21, "st");
    assert_ordinal_suffix(22, "nd");
    assert_ordinal_suffix(23, "rd");
    assert_ordinal_suffix(24, "th");
    assert_ordinal_suffix(100, "th");
    assert_ordinal_suffix(101, "st");
    assert_ordinal_suffix(102, "nd");
    assert_ordinal_suffix(103, "rd");
    assert_ordinal_suffix(104, "th");
    assert_ordinal_suffix(110, "th");
    assert_ordinal_suffix(111, "th");
    assert_ordinal_suffix(112, "th");
    assert_ordinal_suffix(113, "th");
    assert_ordinal_suffix(114, "th");
}

#[cfg(test)]
fn assert_ordinal_suffix(ordinal_position: usize, suffix: &str) {
    assert!(ordinal_position > 0, "Ordinal position must be positive!");
    let err = DecodeError::UnrecognizedWord {
        word: String::from(""),
        position: ordinal_position - 1,
    };
    assert_eq!(
        err.position_as_ordinal_string(),
        format!("{}{}", ordinal_position, suffix)
    );
}
