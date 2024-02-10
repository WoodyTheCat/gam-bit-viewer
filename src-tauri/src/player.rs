use std::{
    cell::RefCell,
    process::{Child, Command, Stdio},
};

use serde::Serialize;

use crate::{engine::Engine, error::EngineError};

const DEFAULT_TIME: u32 = 1000;

pub enum Player {
    Human,
    Engine(Engine),
}

impl Player {
    pub fn new_human() -> Self {
        Self::Human
    }

    pub fn new_engine(path: &str) -> Result<Self, EngineError> {
        Ok(Self::Engine(Engine::new(path)?))
    }

    pub fn is_human(&self) -> bool {
        if let Self::Human = self {
            true
        } else {
            false
        }
    }
    pub fn is_engine(&self) -> bool {
        if let Self::Engine { .. } = self {
            true
        } else {
            false
        }
    }
}
