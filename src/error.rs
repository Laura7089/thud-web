use crate::SessionID;
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
