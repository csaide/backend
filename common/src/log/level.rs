// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Super usings
use super::error::{Error, Result};

// Standard usings
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Level {
    Crit,
    Error,
    Warn,
    Info,
    Debug,
}

impl FromStr for Level {
    type Err = Error;

    fn from_str(t: &str) -> Result<Level> {
        match t {
            "critical" => Ok(Level::Crit),
            "error" => Ok(Level::Error),
            "warn" => Ok(Level::Warn),
            "info" => Ok(Level::Info),
            "debug" => Ok(Level::Debug),
            _ => Err(Error::InvalidLevel {
                level: t.to_owned(),
            }),
        }
    }
}

impl Level {
    pub fn to_slog(&self) -> slog::Level {
        match self {
            Level::Crit => slog::Level::Critical,
            Level::Error => slog::Level::Error,
            Level::Warn => slog::Level::Warning,
            Level::Info => slog::Level::Info,
            Level::Debug => slog::Level::Debug,
        }
    }
}
