use thud::{Coord, ThudError};
use rocket::request::FromForm;

#[derive(FromForm)]
pub struct Move {
    src_x: usize,
    src_y: usize,
    dest_x: usize,
    dest_y: usize,
}

impl Move {
    pub fn into_coords(&self) -> Result<(Coord, Coord), ThudError> {
        let src = Coord::zero_based(self.src_x, self.src_y)?;
        let dest = Coord::zero_based(self.dest_x, self.dest_y)?;
        Ok((src, dest))
    }
}
