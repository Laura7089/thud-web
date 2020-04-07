use crate::SessionID;
use std::error::Error;
use std::fs;
use thud::Thud;

fn path(sessionid: SessionID) -> String {
    format!("game_saves/{}.json", sessionid)
}

pub fn get_game(sessionid: SessionID) -> Result<Thud, Box<dyn Error>> {
    Ok(serde_json::from_reader(fs::File::open(path(sessionid))?)?)
}

pub fn write_game(sessionid: SessionID, thud: &Thud) {
    fs::write(path(sessionid), serde_json::to_string(thud).unwrap()).unwrap();
}

pub fn new_game(sessionid: SessionID) -> bool {
    if fs::metadata(path(sessionid)).is_ok() {
        return false;
    }
    write_game(sessionid, &Thud::new());
    true
}
