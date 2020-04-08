use crate::{error::ThudWebError, JRep, SessionID, ThudResponse};

use rand::prelude::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use thud::{Player, Thud};

type Hash = u64;

#[derive(Serialize, Deserialize)]
pub struct ThudSave {
    id: SessionID,
    troll_hash: Hash,
    dwarf_hash: Hash,
    pub game: Thud,
}

impl ThudSave {
    pub fn verify(&self, pass: &String) -> Result<(), Json<ThudWebError>> {
        let hashed = hash(&pass);
        let pass_player = if hashed == self.dwarf_hash {
            Player::Dwarf
        } else if hashed == self.troll_hash {
            Player::Troll
        } else {
            return Err(Json(ThudWebError::IncorrectPassword));
        };

        let turn_player = self
            .game
            .turn()
            .ok_or(Json(ThudWebError::GameError(thud::ThudError::BadAction)))?;

        if pass_player != turn_player {
            return Err(Json(ThudWebError::GameError(thud::ThudError::BadAction)));
        }
        Ok(())
    }
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

fn hash(input: &String) -> Hash {
    let mut hasher = Sha256::new();
    hasher.input(input.as_bytes());
    // Crappy method of summing to make a string
    let mut sum: Hash = 0;
    hasher.result().iter().for_each(|x| sum += *x as Hash);
    sum
}

pub fn load(sessionid: SessionID) -> Result<ThudSave, Json<ThudWebError>> {
    let file = match fs::File::open(path(sessionid)) {
        Ok(f) => f,
        Err(_) => return Err(Json(ThudWebError::SessionNotFound(sessionid))),
    };
    Ok(serde_json::from_reader(file).unwrap())
}

pub fn write(thud: &ThudSave) -> Result<(), Json<ThudWebError>> {
    let thud_json = serde_json::to_string(thud).unwrap();
    fs::write(path(thud.id), thud_json).or(Err(Json(ThudWebError::Unknown)))
}

pub fn new() -> JRep {
    let mut rng = rand::thread_rng();

    // Get a unique session id
    let mut id: SessionID = 0;
    for i in 0..5 {
        id = rng.gen();
        if !Path::new(&path(id)).exists() {
            break;
        }
        if i == 4 {
            return Err(Json(ThudWebError::Unknown));
        }
    }

    // Generate a new game
    let passes: Vec<_> = vec![gen_pass(&mut rng), gen_pass(&mut rng)];
    let hashes: Vec<_> = passes.iter().map(hash).collect();
    let save = ThudSave {
        id,
        dwarf_hash: hashes[0],
        troll_hash: hashes[1],
        game: Thud::new(),
    };
    write(&save)?;

    // Return
    Ok(Json(ThudResponse::GameCreated {
        id,
        dwarf_pass: passes[0].clone(),
        troll_pass: passes[1].clone(),
    }))
}
