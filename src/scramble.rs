use thiserror::Error;
use std::fmt::Display;
use rand::prelude::*;
use rand::seq::IndexedRandom;

#[derive(Copy, Clone)]
enum Face {
    U,
    D,
    L,
    R,
    F,
    B,
}

#[derive(Error, Debug)]
pub enum MoveCreationError {
    #[error("degree must be 1, 2, or 3")]
    InvalidDegree
}

struct Move {
    face: Face,
    degree: u32,
}

impl Move {
    fn new(face: Face, degree: u32) -> Result<Self, MoveCreationError> {
        match degree {
            1..=3 => Ok(Self { face, degree }),
            _ => Err(MoveCreationError::InvalidDegree) 
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let degree_str = match self.degree {
            1 => "",
            2 => "2",
            3 => "'",
            _ => panic!(),
        };

        let face_str = match self.face {
            Face::U => "U",
            Face::D => "D",
            Face::L => "L",
            Face::R => "R",
            Face::B => "B",
            Face::F => "F",
        };

        write!(f, "{face_str}{degree_str}")
    }
}

const MOVES: [Face; 6] = [Face::U, Face::D, Face::L, Face::R, Face::F, Face::B];
const NUM_MOVES: u32 = 26;

fn random_move() -> Move {
    let mut rng = rand::rng();
    let face = MOVES.choose(&mut rng).unwrap();
    let degree = rng.random_range(1..=3);
    Move::new(*face, degree).unwrap()
}

pub fn random_scramble() -> String {
    (0..NUM_MOVES)
        .map(|_| random_move().to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
