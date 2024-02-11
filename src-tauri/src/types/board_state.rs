use serde::Serialize;

use crate::error::{AppError, EngineError};

use super::{colour::Colour, piece::Piece, position::Position, square_index::SquareIndex};

#[derive(Clone, Copy, Serialize, Default)]
pub struct BoardState {
    active: Colour,
    ep_target: SquareIndex,
    half: i32,
    full: i32,
    pos: Position,
    rights: [bool; 4],
}

impl BoardState {
    pub fn parse_fen(f: &str) -> Result<BoardState, AppError> {
        todo!()
    }

    fn parse_pieces(string: &str) -> Result<Position, AppError> {
        let mut pos = Position::default();

        let s = string.split('/').collect::<Vec<&str>>();

        for (rank, contents) in s.into_iter().enumerate() {
            let mut file = 0;

            for c in contents.chars() {
                match c {
                    c if c.is_alphabetic() => {
                        let piece: Piece = Piece::parse(c)?;
                        pos.set_at(7 - rank as u64, file, Some(piece))
                    }
                    '1'..='8' => {
                        file += char::to_digit(c, 10).unwrap() as u64;
                    }
                    c => return Err(EngineError::InvalidFENChar(c).into()),
                }
                if char::is_alphabetic(c) {
                    file += 1;
                }
            }
        }

        Ok(pos)
    }
}
