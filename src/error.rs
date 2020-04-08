use crate::SessionID;
use serde::Serialize;

#[derive(Serialize, Debug, Copy, Clone)]
pub enum ThudWebError {
    SessionExists(SessionID),
    SessionNotFound(SessionID),
    GameError(thud::ThudError),
    IncorrectPassword,
    Unknown,
}

impl From<thud::ThudError> for ThudWebError {
    fn from(err: thud::ThudError) -> Self {
        Self::GameError(err)
    }
}
