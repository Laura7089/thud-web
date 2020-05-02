use crate::error::Error;
use rocket::request::FromForm;
use serde::Deserialize;
use thud::{Coord, Direction};

#[derive(FromForm, Deserialize)]
pub struct Move {
    pub pass: String,
    x: usize,
    y: usize,
    to_x: usize,
    to_y: usize,
}

impl Move {
    pub fn into_coords(&self) -> Result<(Coord, Coord), Error> {
        let src = Coord::zero_based(self.x, self.y)?;
        let dest = Coord::zero_based(self.to_x, self.to_y)?;
        Ok((src, dest))
    }

    pub fn try_move(&self, game: &mut thud::Thud) -> Result<(), Error> {
        let (src, dest) = self.into_coords()?;
        game.move_piece(src, dest)?;
        Ok(())
    }

    pub fn try_attack(&self, game: &mut thud::Thud) -> Result<(), Error> {
        let (src, dest) = self.into_coords()?;
        game.attack(src, dest)?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct TrollTake {
    pub pass: String,
    x: usize,
    y: usize,
    targets: Vec<u8>,
}

impl TrollTake {
    pub fn try_take(&self, game: &mut thud::Thud) -> Result<(), Error> {
        let coord = Coord::zero_based(self.x, self.y)?;
        let mut dirs: Vec<Direction> = Vec::with_capacity(8);
        for target in self.targets.iter() {
            let dir = Direction::from_num(*target as usize)?;
            dirs.push(dir);
        }
        game.troll_cap(coord, dirs)?;
        Ok(())
    }
}
