use crate::error::ThudError;
use crate::SessionID;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use thud::Thud;

#[derive(Serialize, Deserialize)]
pub struct ThudSave {
    id: SessionID,
    pub game: Thud,
}

fn path(sessionid: SessionID) -> String {
    format!("game_saves/{}.json", sessionid)
}

pub fn load(sessionid: SessionID) -> Result<ThudSave, ThudError> {
    let file = match fs::File::open(path(sessionid)) {
        Ok(f) => f,
        Err(_) => return Err(ThudError::SessionNotFound(sessionid)),
    };
    Ok(serde_json::from_reader(file).unwrap())
}

pub fn write(sessionid: SessionID, thud: &ThudSave) -> Result<(), ThudError> {
    let thud_json = serde_json::to_string(thud).unwrap();
    match fs::write(path(sessionid), thud_json) {
        Ok(_) => Ok(()),
        Err(_) => Err(ThudError::Unknown),
    }
}

pub fn new(sessionid: SessionID) -> Result<(), ThudError> {
    if !fs::metadata(path(sessionid)).is_err() {
        return Err(ThudError::SessionExists(sessionid));
    }
    write(
        sessionid,
        &ThudSave {
            id: sessionid,
            game: Thud::new(),
        },
    )?;
    Ok(())
}
