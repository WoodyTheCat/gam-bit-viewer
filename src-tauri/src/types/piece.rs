use serde::Serialize;

use crate::error::{AppError, EngineError};

#[derive(Clone, Copy, Serialize)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn parse(c: char) -> Result<Self, AppError> {
        Ok(match c {
            'P' => Self::WhitePawn,
            'N' => Self::WhiteKnight,
            'B' => Self::WhiteBishop,
            'R' => Self::WhiteRook,
            'Q' => Self::WhiteQueen,
            'K' => Self::WhiteKing,
            'p' => Self::BlackPawn,
            'n' => Self::BlackKnight,
            'b' => Self::BlackBishop,
            'r' => Self::BlackRook,
            'q' => Self::BlackQueen,
            'k' => Self::BlackKing,
            e => {
                return Err(AppError::EngineError(EngineError::InvalidFENChar(e)));
            }
        })
    }
}
