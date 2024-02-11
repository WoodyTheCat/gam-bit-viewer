// #[derive(Serialize)]
// pub struct AppError(String);

use serde::Serialize;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum AppError {
    #[error("Mutex was poisoned!")]
    PoisonError,
    // #[error("")]
    // Io {
    //     #[from]
    //     source: std::io::Error,
    //     backtrace: Backtrace,
    // },
    #[error("Tauri error")]
    Tauri,
    #[error("Engine error: {0:?}")]
    EngineError(#[from] EngineError),
    #[error("{0}")]
    String(String),
}

// impl From<tauri::Error> for AppError {
//     fn from(value: tauri::Error) -> Self {
//         Self(value)
//     }
// }

#[derive(thiserror::Error, Debug, Serialize)]
pub enum EngineError {
    #[error("Connection failed!")]
    ConnectionFailure,
    #[error("Io error")]
    Io,
    #[error("Invalid option \"{0}\" with value \"{1}\"!")]
    InvalidOption(String, String),
    #[error("Invalid FEN character: '{0}'")]
    InvalidFENChar(char),
}
