use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Parity {
    Even,
    Odd,
}

impl fmt::Display for Parity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Even => write!(f, "even"),
            Self::Odd => write!(f, "odd"),
        }
    }
}

impl Parity {
    fn other(self) -> Self {
        match self {
            Self::Even => Self::Odd,
            Self::Odd => Self::Even,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SwappingParity {
    pub parity: Parity,
}

impl SwappingParity {
    pub fn new(initial: Parity) -> Self {
        Self { parity: initial }
    }

    pub fn current_value_with_swap_side_effect(&mut self) -> Parity {
        let current = self.parity;
        self.parity = self.parity.other();
        current
    }

    pub fn other(self) -> Parity {
        self.parity.other()
    }
}
