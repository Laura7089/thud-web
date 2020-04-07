#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod game_move;
mod saves;
use crate::game_move::Move;

use rocket::request::Form;
use rocket_contrib::json::Json;
use serde::Serialize;
use thud::Thud;

type SessionID = u32;

#[derive(Serialize)]
enum ReqResult {
    Board(thud::Board),
    GameOver(thud::EndState, thud::Board),
    SessionExists,
    Success,
    Unknown,
    SessionNotFound,
    BadCoordinate,
    IllegalMove(thud::ThudError),
}

#[get("/board?<sessionid>")]
fn game_state(sessionid: SessionID) -> Json<ReqResult> {
    Json(match saves::get_game(sessionid) {
        Ok(mut game) => {
            if let Some(state) = game.winner() {
                saves::write_game(sessionid, &game);
                ReqResult::GameOver(state, game.board())
            } else {
                ReqResult::Board(game.board())
            }
        }
        Err(_) => ReqResult::NoSuchSession,
    })
}

#[post("/move?<sessionid>", data = "<wanted_move>")]
fn move_piece(sessionid: SessionID, wanted_move: Form<Move>) -> Json<ReqResult> {
    let mut game = match saves::get_game(sessionid) {
        Ok(g) => g,
        Err(_) => return Json(ReqResult::NoSuchSession),
    };

    let (src, dest) = match wanted_move.into_coords() {
        Ok(c) => c,
        Err(_) => return Json(ReqResult::BadCoordinate),
    };

    Json(match game.move_piece(src, dest) {
        Ok(_) => {
            saves::write_game(sessionid, &game);
            ReqResult::Ok
        }
        Err(e) => ReqResult::IllegalMove(e),
    })
}

#[post("/new?<sessionid>")]
fn new_game(sessionid: SessionID) -> Json<ReqResult> {
    Json(if saves::new_game(sessionid) {
        ReqResult::Ok
    } else {
        ReqResult::SessionExists
    })
}

fn main() {
    let thud = Thud::new();
    saves::write_game(100, &thud);
    rocket::ignite()
        .mount("/thud", routes![game_state, move_piece, new_game])
        .launch();
}
