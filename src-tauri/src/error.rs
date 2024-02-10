use std::sync::PoisonError;

use serde::Serialize;

#[derive(Serialize)]
pub struct AppError(pub String);

impl From<EngineError> for AppError {
    fn from(value: EngineError) -> Self {
        Self(value.0)
    }
}

impl<T> From<PoisonError<T>> for AppError {
    fn from(value: PoisonError<T>) -> Self {
        Self(String::from("Poison error!"))
    }
}

#[derive(Debug)]
pub struct EngineError(pub String);

impl From<std::io::Error> for EngineError {
    fn from(_value: std::io::Error) -> Self {
        Self(String::from("Stdio error"))
    }
}
