use crate::SessionID;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize, Debug, Copy, Clone)]
pub enum Error {
    SessionExists(SessionID),
    SessionNotFound(SessionID),
    GameError(thud::ThudError),
    IncorrectPassword,
    Unknown,
}

impl From<thud::ThudError> for Error {
    fn from(err: thud::ThudError) -> Self {
        Self::GameError(err)
    }
}

impl From<Error> for Json<Error> {
    fn from(e: Error) -> Self {
        Json(e)
    }
}

// impl From<thud::ThudError> for Json<Error> {
//     fn from(e: thud::ThudError) -> Self {
//         Json(Error::GameError(e))
//     }
// }
