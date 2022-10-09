use std::error::Error;

use std::{fmt, fmt::Display};
#[derive(Debug)]
pub enum ParseErr {
    Malformed,
    Empty,
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Todo list reading failed")
    }
}

impl Error for ReadErr {
    fn description(&self) -> &str {
        "Todo list read failed: "
    }
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Todo list parsing failed")
    }
}

impl Error for ParseErr {
    fn description(&self) -> &str {
        "Todo list parse failed: "
    }
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
