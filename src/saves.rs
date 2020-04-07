use crate::SessionID;
use crate::ThudError;
use rocket_contrib::json::Json;
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

pub fn load(sessionid: SessionID) -> Result<ThudSave, Json<ThudError>> {
    let file = match fs::File::open(path(sessionid)) {
        Ok(f) => f,
        Err(_) => return Err(Json(ThudError::SessionNotFound(sessionid))),
    };
    Ok(serde_json::from_reader(file).unwrap())
}

pub fn write(sessionid: SessionID, thud: &ThudSave) -> Result<(), Json<ThudError>> {
    let thud_json = serde_json::to_string(thud).unwrap();
    match fs::write(path(sessionid), thud_json) {
        Ok(_) => Ok(()),
        Err(_) => Err(Json(ThudError::Unknown)),
    }
}

pub fn new(sessionid: SessionID) -> Result<(), Json<ThudError>> {
    if !fs::metadata(path(sessionid)).is_err() {
        return Err(Json(ThudError::SessionExists(sessionid)));
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
