use crate::ThudError;
use crate::ThudError::BadCoordinate;
use rocket::request::FromForm;
use rocket_contrib::json::Json;
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
    pub fn into_coords(&self) -> Result<(Coord, Coord), Json<ThudError>> {
        let src = match Coord::zero_based(self.x, self.y) {
            Ok(c) => c,
            Err(_) => return Err(Json(BadCoordinate(self.x, self.y))),
        };
        let dest = match Coord::zero_based(self.to_x, self.to_y) {
            Ok(c) => c,
            Err(_) => return Err(Json(BadCoordinate(self.to_x, self.to_y))),
        };
        Ok((src, dest))
    }
}
