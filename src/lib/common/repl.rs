use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use reedline_repl_rs::clap::ArgMatches;
use crate::common::repl::AppError::{ReplError, StringError};

#[derive(Debug)]
pub enum AppError {
    ReplError(reedline_repl_rs::Error),
    StringError(String),
}

impl From<reedline_repl_rs::Error> for AppError {
    fn from(e: reedline_repl_rs::Error) -> Self {
        ReplError(e)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplError(e) => write!(f, "REPL Error: {}", e),
            StringError(s) => write!(f, "String Error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

pub fn get_param<T: Clone + Send + Sync + FromStr>(args: &ArgMatches, name: &str) -> Result<T, AppError> {
    match args.get_one::<String>(name) {
        None => Err(StringError(format!("Missing parameter {}", name))),
        Some(v) => v.parse::<T>().or(Err(StringError("Error parsing parameter".to_string()))),
    }
}