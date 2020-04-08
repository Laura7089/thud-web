use crate::error::ThudWebError;
use rocket::request::FromForm;
use rocket_contrib::json::Json;
use serde::Deserialize;
use thud::{Coord, Direction};

type MoveRes = Result<(), Json<ThudWebError>>;

#[derive(FromForm, Deserialize)]
pub struct Move {
    pub pass: String,
    x: usize,
    y: usize,
    to_x: usize,
    to_y: usize,
}

impl Move {
    pub fn into_coords(&self) -> Result<(Coord, Coord), Json<ThudWebError>> {
        let src = Coord::zero_based(self.x, self.y).or_else(|e| Err(Json(e.into())))?;
        let dest = Coord::zero_based(self.to_x, self.to_y).or_else(|e| Err(Json(e.into())))?;
        Ok((src, dest))
    }

    pub fn try_move(&self, game: &mut thud::Thud) -> MoveRes {
        let (src, dest) = self.into_coords()?;
        game.move_piece(src, dest)
            .or_else(|e| Err(Json(e.into())))?;
        Ok(())
    }

    pub fn try_attack(&self, game: &mut thud::Thud) -> MoveRes {
        let (src, dest) = self.into_coords()?;
        game.attack(src, dest).or_else(|e| Err(Json(e.into())))?;
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
    pub fn try_take(&self, game: &mut thud::Thud) -> MoveRes {
        let coord = Coord::zero_based(self.x, self.y).or_else(|e| Err(Json(e.into())))?;
        let mut dirs: Vec<Direction> = Vec::with_capacity(8);
        for target in self.targets.iter() {
            let dir = Direction::from_num(*target as usize).or_else(|e| Err(Json(e.into())))?;
            dirs.push(dir);
        }

        game.troll_cap(coord, dirs)
            .or_else(|e| Err(Json(e.into())))?;

        Ok(())
    }
}
