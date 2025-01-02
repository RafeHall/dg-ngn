#![feature(new_range_api)]

// pub mod command_parser_combinator;
pub mod command;
use core::range::Range;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ParseErrorKind {
    #[error("unexpected end of input")]
    UnexpectedEoi,
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("parse error")]
    ParseError {
        span: Range<usize>,
        kind: ParseErrorKind,
    }
}

pub struct Console {

}

impl Console {
    pub fn run(&self, command: &str) -> Result<(), Error> {
        let command = command::parse_command(command);
        
        Ok(())
    }
}