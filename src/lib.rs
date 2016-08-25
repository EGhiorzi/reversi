//! The `reversi` library (ver. 0.1.0) provides the main structures and mechanics required to run a Reversi game.
//! In view of possible AIs developement, the library keeps an eye on performances.
//! Released under MIT license.

// Roadmap to 0.1.0:
// ReversiError::NoUndo needs a turn::Turn value
// board::Board needs IntoIter so that its (unique) component can be made private

#![crate_name = "reversi"]
#![crate_type = "lib"]

pub mod board;
pub mod turn;
pub mod game;

use std::{error, fmt, result};
use board::{Coord, Direction};

/// The errors that may be generated by running a Reversi game.
#[derive(Debug, Clone, Copy)]
pub enum ReversiError {
    /// It has been attempted to create or use a coordinate with out-of-bound indexes.
    OutOfBoundCoord(Coord),
    /// It has been attempted to step out of the board's bounds.
    OutOfBoundStep(Coord, Direction),
    /// It has been attempted to move on a cell which was already taken.
    CellAlreadyTaken(Coord),
    /// It was looked for a disk in a cell which is empty.
    EmptyCell(Coord),
    /// It has been attempted to move on a illegal cell.
    IllegalMove(Coord),
    /// Undoing a turn is not possible
    NoUndo,
    /// It has been tried to move when the game was already ended.
    EndedGame,
}

/// Aliasing given by taking `ReversiError` as standard error value.
pub type Result<T> = result::Result<T, ReversiError>;

impl fmt::Display for ReversiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReversiError::OutOfBoundCoord(coord) => write!(f, "Out of bound coordinates: {:?}", coord),
            ReversiError::OutOfBoundStep(coord, dir) => write!(f, "Out of bound step: {:?} going {:?}", coord, dir),
            ReversiError::CellAlreadyTaken(coord) => write!(f, "The cell you want to move to is already taken: {:?}", coord),
            ReversiError::IllegalMove(coord) => write!(f, "Illegal move: {:?}", coord),
            ReversiError::EmptyCell(coord) => write!(f, "The cell you want is empty: {:?}", coord),
            ReversiError::NoUndo => write!(f, "Undoing is not possible"),
            ReversiError::EndedGame => write!(f, "The game is already ended"),
        }
    }
}

impl error::Error for ReversiError {
    fn description(&self) -> &str {
        match *self {
            ReversiError::OutOfBoundCoord(_) => "Out of bound coordinates",
            ReversiError::OutOfBoundStep(_, _) => "Out of bound step",
            ReversiError::CellAlreadyTaken(_) => "The cell you want to move to is already taken",
            ReversiError::IllegalMove(_) => "Illegal move",
            ReversiError::EmptyCell(_) => "The cell you want is empty",
            ReversiError::NoUndo => "Undoing is not possible",
            ReversiError::EndedGame => "The game is already ended",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// There are two sides in Reversi: `Dark` and `Light`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Dark,
    Light,
}

impl Side {
    /// Get self's opposite side.
    pub fn opposite(&self) -> Side {
        match *self {
            Side::Dark  => Side::Light,
            Side::Light => Side::Dark,
        }
    }
}
