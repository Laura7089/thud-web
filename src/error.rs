use crate::SessionID;
use crate::ThudResponse;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub enum ThudError {
    SessionExists(SessionID),
    SessionNotFound(SessionID),
    BadCoordinate(usize, usize),
    BadMove(thud::ThudError),
    Unknown,
}

impl ThudError {
    pub fn wrap(self) -> Json<ThudResponse> {
        Json(ThudResponse::Err(self))
    }
}
