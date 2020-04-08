#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod interact;
mod saves;

use rocket_contrib::json::Json;
use serde::Serialize;

type SessionID = u32;
type Password = String;
type JRep = Result<Json<ThudResponse>, Json<ThudError>>;

#[derive(Serialize)]
pub enum ThudResponse {
    Success,
    GameCreated {
        id: SessionID,
        dwarf_pass: Password,
        troll_pass: Password,
    },
    Board(thud::Board),
    GameOver(thud::EndState, thud::Board),
}

#[derive(Serialize, Debug)]
pub enum ThudError {
    SessionExists(SessionID),
    SessionNotFound(SessionID),
    BadCoordinate(usize, usize),
    BadMove(thud::ThudError),
    Unknown,
}

#[get("/board?<sessionid>")]
fn game_state(sessionid: SessionID) -> JRep {
    // Get the save
    let mut save = saves::load(sessionid)?;

    // Calculate the winner and save the game so it's cached
    let winner = save.game.winner();
    saves::write(sessionid, &save)?;

    // Return the result
    Ok(Json(if let Some(state) = winner {
        ThudResponse::GameOver(state, save.game.board())
    } else {
        ThudResponse::Board(save.game.board())
    }))
}

#[post("/move?<sessionid>", data = "<wanted_move>")]
fn move_piece(sessionid: SessionID, wanted_move: Json<interact::Move>) -> JRep {
    // Process the coordinates for the move
    let (src, dest) = wanted_move.into_coords()?;

    // Load the session save
    let mut save = saves::load(sessionid)?;

    // Try to move the piece and report the result
    match save.game.move_piece(src, dest) {
        Ok(_) => {
            saves::write(sessionid, &save)?;
            Ok(Json(ThudResponse::Success))
        }
        Err(e) => Err(Json(ThudError::BadMove(e))),
    }
}

#[get("/new")]
fn new() -> JRep {
    saves::new()
}

fn main() {
    rocket::ignite()
        .mount("/thud", routes![game_state, move_piece, new])
        .launch();
}
