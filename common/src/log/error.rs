// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Super usings
use super::serde;

// Standard usings
use serde::ser::SerializeStruct;
use std::{error, fmt, io, result};

static KIND: &str = "log-error";

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, SerdeValue)]
pub enum Error {
    IOError {
        path: String,
        err: std::sync::Arc<io::Error>,
    },
    HandlerMissing {
        handler: String,
    },
    InvalidLevel {
        level: String,
    },
}

impl Error {
    fn message(&self) -> &str {
        match self {
            Error::IOError { .. } => "Could not open configured log path.",
            Error::HandlerMissing { .. } => "Specified log handler is invalid.",
            Error::InvalidLevel { .. } => "Specified log level is invalid.",
        }
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Error::IOError { ref path, ref err } => {
                let mut sv = serializer.serialize_struct("LogError", 4)?;
                sv.serialize_field("kind", KIND)?;
                sv.serialize_field("description", self.message())?;
                sv.serialize_field("path", path)?;
                sv.serialize_field("cause", &format!("{}", err))?;
                sv.end()
            }
            Error::HandlerMissing { ref handler } => {
                let mut sv = serializer.serialize_struct("LogError", 3)?;
                sv.serialize_field("kind", KIND)?;
                sv.serialize_field("description", self.message())?;
                sv.serialize_field("handler", handler)?;
                sv.end()
            }
            Error::InvalidLevel { ref level } => {
                let mut sv = serializer.serialize_struct("LogError", 3)?;
                sv.serialize_field("kind", KIND)?;
                sv.serialize_field("description", self.message())?;
                sv.serialize_field("level", level)?;
                sv.end()
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOError { ref path, ref err } => {
                write!(f, "could not open configured log path '{}': {}", path, err)
            }
            Error::HandlerMissing { ref handler } => {
                write!(f, "specified log handler '{}' is not implemented", handler)
            }
            Error::InvalidLevel { ref level } => {
                write!(f, "specified log level '{}' is not implemented", level)
            }
        }
    }
}

impl slog::KV for Error {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("error", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        KIND
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
