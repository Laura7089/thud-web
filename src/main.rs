#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod error;
mod game_move;
mod saves;
use crate::error::ThudError;
use crate::game_move::Move;

use rocket_contrib::json::Json;
use serde::Serialize;

type SessionID = u32;
type JRep = Json<ThudResponse>;

#[derive(Serialize)]
pub enum ThudResponse {
    Success,
    Board(thud::Board),
    GameOver(thud::EndState, thud::Board),
    Err(ThudError),
}

#[get("/board?<sessionid>")]
fn game_state(sessionid: SessionID) -> JRep {
    // Get the save
    let mut save = match saves::load(sessionid) {
        Ok(save) => save,
        Err(e) => return e.wrap(),
    };

    // Calculate the winner and save the game so it's cached
    let winner = save.game.winner();
    if let Err(e) = saves::write(sessionid, &save) {
        return e.wrap();
    }

    // Return the result
    Json(if let Some(state) = winner {
        ThudResponse::GameOver(state, save.game.board())
    } else {
        ThudResponse::Board(save.game.board())
    })
}

#[post("/move?<sessionid>", data = "<wanted_move>")]
fn move_piece(sessionid: SessionID, wanted_move: Json<Move>) -> JRep {
    // Process the coordinates for the move
    let (src, dest) = match wanted_move.into_coords() {
        Ok(c) => c,
        Err(e) => return e.wrap(),
    };

    // Load the session save
    let mut save = match saves::load(sessionid) {
        Ok(g) => g,
        Err(e) => return e.wrap(),
    };

    // Try to move the piece and report the result
    match save.game.move_piece(src, dest) {
        Ok(_) => {
            if let Err(e) = saves::write(sessionid, &save) {
                e.wrap()
            } else {
                Json(ThudResponse::Success)
            }
        }
        Err(e) => ThudError::BadMove(e).wrap(),
    }
}

#[post("/new?<sessionid>")]
fn new(sessionid: SessionID) -> JRep {
    match saves::new(sessionid) {
        Ok(_) => Json(ThudResponse::Success),
        Err(e) => e.wrap(),
    }
}

fn main() {
    rocket::ignite()
        .mount("/thud", routes![game_state, move_piece, new])
        .launch();
}
