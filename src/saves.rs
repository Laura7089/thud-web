use crate::{SessionID, ThudError, ThudResponse};

use rand::prelude::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use thud::Thud;

type Hash = u64;

#[derive(Serialize, Deserialize)]
pub struct ThudSave {
    id: SessionID,
    pub troll_hash: Hash,
    pub dwarf_hash: Hash,
    pub game: Thud,
}

fn path(sessionid: SessionID) -> String {
    format!(
        "{}/{}.json",
        std::env::var("THUD_SAVES_DIR").unwrap_or("game_saves".into()),
        sessionid
    )
}

fn gen_pass(rng: &mut ThreadRng) -> String {
    let base = 2_u32;
    let pass_raw: u32 = rng.gen_range(base.pow(17), (base.pow(20)) - 1);
    format!("{:x}", pass_raw)
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

pub fn new() -> Result<Json<ThudResponse>, Json<ThudError>> {
    let mut rng = rand::thread_rng();

    // Get a unique session id
    let mut id: SessionID = 0;
    for i in 0..5 {
        id = rng.gen();
        if !Path::new(&path(id)).exists() {
            break;
        }
        if i == 4 {
            return Err(Json(ThudError::Unknown));
        }
    }

    // Generate passwords and hashes
    let (troll_pass, dwarf_pass) = (gen_pass(&mut rng), gen_pass(&mut rng));
    let mut hashes: Vec<Hash> = Vec::with_capacity(2);
    for pass in &[&dwarf_pass, &troll_pass] {
        let mut hasher = Sha256::new();
        hasher.input(pass.as_bytes());
        // Crappy method of summing to make a string
        let mut sum: Hash = 0;
        hasher.result().iter().for_each(|x| sum += *x as Hash);
        hashes.push(sum);
    }

    // Write to file
    write(
        id,
        &ThudSave {
            id,
            dwarf_hash: hashes[0],
            troll_hash: hashes[1],
            game: Thud::new(),
        },
    )
    .unwrap();

    // Return
    Ok(Json(ThudResponse::GameCreated {
        id,
        dwarf_pass,
        troll_pass,
    }))
}
