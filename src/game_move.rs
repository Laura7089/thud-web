use crate::error::ThudError;
use crate::error::ThudError::BadCoordinate;
use rocket::request::FromForm;
use serde::Deserialize;
use thud::Coord;

#[derive(FromForm, Deserialize)]
pub struct Move {
    x: usize,
    y: usize,
    to_x: usize,
    to_y: usize,
}

impl Move {
    pub fn into_coords(&self) -> Result<(Coord, Coord), ThudError> {
        let src = match Coord::zero_based(self.x, self.y) {
            Ok(c) => c,
            Err(_) => return Err(BadCoordinate(self.x, self.y)),
        };
        let dest = match Coord::zero_based(self.to_x, self.to_y) {
            Ok(c) => c,
            Err(_) => return Err(BadCoordinate(self.to_x, self.to_y)),
        };
        Ok((src, dest))
    }
}
