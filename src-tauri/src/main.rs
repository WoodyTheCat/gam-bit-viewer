// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, MutexGuard};

use error::AppError;
use player::Player;
use tauri::{AppHandle, Manager, State};
use types::board_state::BoardState;

mod engine;
mod error;
mod player;
mod types;

#[derive(Default)]
pub struct GameState {
    player0: Option<Player>,
    player1: Option<Player>,
    board: BoardState,
}

#[tauri::command]
fn get_board(game_state: State<'_, Mutex<GameState>>) -> Result<BoardState, AppError> {
    let state: MutexGuard<'_, GameState> = game_state.lock().map_err(|_| AppError::PoisonError)?;

    Ok(state.board)
}

#[tauri::command]
fn set_board(
    game_state: State<'_, Mutex<GameState>>,
    handle: AppHandle,
    fen: Option<&str>,
    pgn: Option<&str>,
) -> Result<(), AppError> {
    let mut state: MutexGuard<'_, GameState> =
        game_state.lock().map_err(|_| AppError::PoisonError)?;

    let new_board: BoardState = match (pgn, fen) {
        (Some(_p), None) => {
            let board = BoardState::default();
            // let moves: Vec<Move> = BoardState::parse_pgn();

            // for mv in moves {
            // board.apply_move();
            // }

            board
        }
        (None, Some(f)) => BoardState::parse_fen(f)?,
        _ => {
            return Err(AppError::String(
                "Too few or many arguments to set_board command.".into(),
            ));
        }
    };

    state.board = new_board;

    handle
        .emit_all("set_board", new_board)
        .map_err(|_| AppError::Tauri)?;

    Ok(())
}

#[tauri::command]
fn start(handle: AppHandle) -> Result<(), AppError> {
    handle
        .emit_all("refresh_board", "some unique payload")
        .map_err(|_| AppError::Tauri)?;

    Ok(())
}

fn main() {
    // let menu = Menu::new()
    //     .add_native_item(MenuItem::Copy)
    //     .add_item(CustomMenuItem::new("hide", "Hide"));

    tauri::Builder::default()
        .manage::<Mutex<GameState>>(Mutex::new(GameState::default()))
        // .menu(menu)
        .invoke_handler(tauri::generate_handler![get_board, set_board, start])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application!");
}
