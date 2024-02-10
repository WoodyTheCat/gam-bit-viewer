// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex, MutexGuard};

use error::AppError;
// use player::Player;
use tauri::State;
use types::board_state::BoardState;

mod engine;
mod error;
mod player;
mod types;

pub struct GameState {
    // player1: Option<Player>,
    // player2: Option<Player>,
    board: BoardState,
}

pub struct ManagedGameState(Mutex<GameState>);

#[tauri::command]
fn get_board(game_state: State<'_, ManagedGameState>) -> Result<BoardState, AppError> {
    let state: MutexGuard<'_, GameState> = game_state.0.lock().unwrap();

    Ok(state.board)
}

#[tauri::command]
fn set_board(game_state: State<'_, ManagedGameState>) -> Result<(), AppError> {
    let state: MutexGuard<'_, GameState> = game_state.0.lock().unwrap();

    Ok(())
}

// #[tauri::command]
// fn new_player(is_first: bool, is_engine: bool, path: Option<&str>) -> Result<Player, AppError> {
//     let player = if is_engine {
//         let Some(path) = path else {
//             return Err(AppError(String::from(
//                 "Engine specified but no path provided!",
//             )));
//         };

//         Player::new_engine(path)?
//     } else {
//         Player::new_human()
//     };

//     Ok(player)
// }

fn main() {
    tauri::Builder::default()
        .manage(ManagedGameState)
        .invoke_handler(tauri::generate_handler![get_board, set_board])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application!");
}
