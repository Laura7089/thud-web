#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod error;
mod interact;
mod saves;

use rocket_contrib::json::Json;
use serde::Serialize;

type SessionID = u32;
type Password = String;
pub type JRep = Result<Json<ThudResponse>, Json<error::ThudWebError>>;

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

#[get("/board?<sessionid>")]
fn game_state(sessionid: SessionID) -> JRep {
    // Calculate the winner and save the game so it's cached
    let mut save = saves::load(sessionid)?;
    let winner = save.game.winner();
    saves::write(&save)?;

    // Return the result
    Ok(Json(if let Some(state) = winner {
        ThudResponse::GameOver(state, save.game.board())
    } else {
        ThudResponse::Board(save.game.board())
    }))
}

#[post("/move?<sessionid>", data = "<wanted_move>")]
fn move_piece(sessionid: SessionID, wanted_move: Json<interact::Move>) -> JRep {
    // Load the session save and verify with the password
    let mut save = saves::load(sessionid)?;
    save.verify(&wanted_move.pass)?;

    // Try the move and write the changes
    wanted_move.try_move(&mut save.game)?;
    saves::write(&save)?;
    Ok(Json(ThudResponse::Success))
}

#[post("/attack?<sessionid>", data = "<wanted_attack>")]
fn attack(sessionid: SessionID, wanted_attack: Json<interact::Move>) -> JRep {
    let mut save = saves::load(sessionid)?;
    save.verify(&wanted_attack.pass)?;

    wanted_attack.try_attack(&mut save.game)?;
    saves::write(&save)?;
    Ok(Json(ThudResponse::Success))
}

#[post("/trolltake?<sessionid>", data = "<targets>")]
fn troll_take(sessionid: SessionID, targets: Json<interact::TrollTake>) -> JRep {
    let mut save = saves::load(sessionid)?;
    save.verify(&targets.pass)?;

    targets.try_take(&mut save.game)?;
    saves::write(&save)?;

    Ok(Json(ThudResponse::Success))
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
