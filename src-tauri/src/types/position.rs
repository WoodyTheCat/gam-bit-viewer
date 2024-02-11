use serde::Serialize;

use super::{piece::Piece, square_index::SquareIndex};

#[derive(Clone, Copy, Serialize, Default)]
pub struct Position(pub [[Option<Piece>; 8]; 8]);

impl Position {
    pub fn at(&self, sq: SquareIndex) -> Option<Piece> {
        let rank: usize = (sq >> 3) as usize;
        let file: usize = (sq & 7) as usize;

        self.0[rank][file]
    }

    pub fn set_at(&mut self, rank: u64, file: u64, new: Option<Piece>) {
        self.0[rank as usize][file as usize] = new;
    }

    pub fn set_at_index(&mut self, sq: SquareIndex, new: Option<Piece>) {
        let rank = sq >> 3;
        let file = sq & 7;
        self.0[rank as usize][file as usize] = new;
    }
}
