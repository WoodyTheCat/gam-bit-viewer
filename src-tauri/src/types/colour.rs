use std::ops::Not;

use serde::Serialize;

#[derive(Clone, Copy, Serialize, Default)]
pub enum Colour {
    #[default]
    White,
    Black,
}

impl Not for Colour {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
