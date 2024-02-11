use std::process::{Child, Command, Stdio};

use std::io::Read;
use std::io::Write;

use std::fmt;
use std::thread;
use std::time::Duration;

use std::cell::RefCell;

use serde::Serialize;

use crate::error::EngineError;

pub struct Engine {
    engine: RefCell<Child>,

    movetime: u32,
}

const DEFAULT_TIME: u32 = 1000;

impl Engine {
    pub fn new(path: &str) -> Result<Engine, EngineError> {
        let cmd = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Unable to run engine");

        let res = Engine {
            engine: RefCell::new(cmd),
            movetime: DEFAULT_TIME,
        };

        res.read_line()?;
        res.command("uci")?;

        Ok(res)
    }

    pub fn movetime(&mut self, new: u32) {
        self.movetime = new;
    }

    pub fn moves_from_start(&self, moves: &[&str]) -> Result<(), EngineError> {
        self.write_fmt(format_args!(
            "position startpos moves {}\n",
            moves.join(" ")
        ))?;
        Ok(())
    }

    pub fn set_position(&self, fen: &str) -> Result<(), EngineError> {
        let moves: Vec<String> = vec![];
        self.make_moves_from_position(fen, &moves)
    }

    pub fn make_moves_from_position(
        &self,
        fen: &str,
        moves: &Vec<String>,
    ) -> Result<(), EngineError> {
        self.write_fmt(format_args!(
            "position fen {} moves {}\n",
            fen,
            moves.join(" ")
        ))?;

        Ok(())
    }

    /// Returns the best move in the current position according to the engine
    pub fn bestmove(&self) -> Result<String, EngineError> {
        self.write_fmt(format_args!("go movetime {}\n", self.movetime))?;
        loop {
            let s = self.read_line()?;
            if s.starts_with("bestmove") {
                return Ok(s.split(" ").collect::<Vec<&str>>()[1].trim().to_string());
            }
        }
    }

    /// Sets an engine specific option to the given value
    ///
    /// # Arguments
    ///
    /// * `name`  - Name of the option
    /// * `value` - New value for the option
    ///
    pub fn set_option(&self, name: &str, value: &str) -> Result<(), EngineError> {
        self.write_fmt(format_args!("setoption name {} value {}\n", name, value))?;
        let error_msg = self.read_left_output()?;

        if error_msg.trim().is_empty() {
            Ok(())
        } else {
            Err(EngineError::InvalidOption(name.into(), value.into()))
        }
    }

    pub fn command(&self, cmd: &str) -> Result<String, EngineError> {
        self.write_fmt(format_args!("{}\n", cmd.trim()))?;
        thread::sleep(Duration::from_millis(100));
        self.read_left_output()
    }

    fn read_left_output(&self) -> Result<String, EngineError> {
        let mut s: Vec<String> = vec![];

        self.write_fmt(format_args!("isready\n"))?;
        loop {
            let next_line = self.read_line()?;
            match next_line.trim() {
                "readyok" => return Ok(s.join("\n")),
                other => s.push(other.to_string()),
            }
        }
    }

    fn write_fmt(&self, args: fmt::Arguments) -> Result<(), EngineError> {
        // info!("Command: {:?}", fmt::format(args));
        self.engine
            .borrow_mut()
            .stdin
            .as_mut()
            .unwrap()
            .write_fmt(args)
            .map_err(|_| EngineError::Io)?;
        Ok(())
    }

    fn read_line(&self) -> Result<String, EngineError> {
        let mut s = String::new();
        let mut buf: Vec<u8> = vec![0];

        loop {
            self.engine
                .borrow_mut()
                .stdout
                .as_mut()
                .unwrap()
                .read(&mut buf)
                .map_err(|_| EngineError::Io)?;
            s.push(buf[0] as char);
            if buf[0] == '\n' as u8 {
                break;
            }
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), EngineError> {
        let mut engine = Engine::new("stockfish")?;
        engine.movetime(200);
        engine.set_option("Skill Level", "15")?;
        let t = engine.bestmove().unwrap();

        println!("{}", t);

        Ok(())
    }
}
