// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Super usings
use serde;

// Standard usings
use serde::ser::SerializeStruct;
use std::{error, fmt, io, result};

static KIND: &str = "rest-error";

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, SerdeValue)]
pub enum Error {
    BindError {
        addr: String,
        err: std::sync::Arc<io::Error>,
    },
    RunError {
        err: std::sync::Arc<io::Error>,
    },
}

impl Error {
    fn message(&self) -> &str {
        match self {
            Error::BindError { .. } => "Could not bind TCP sockets to specified local address.",
            Error::RunError { .. } => "Could not start the HTTP server.",
        }
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Error::BindError { ref addr, ref err } => {
                let mut sv = serializer.serialize_struct("Error", 4)?;
                sv.serialize_field("kind", KIND)?;
                sv.serialize_field("description", self.message())?;
                sv.serialize_field("addr", addr)?;
                sv.serialize_field("cause", &format!("{}", err))?;
                sv.end()
            }
            Error::RunError { ref err } => {
                let mut sv = serializer.serialize_struct("Error", 4)?;
                sv.serialize_field("kind", KIND)?;
                sv.serialize_field("description", self.message())?;
                sv.serialize_field("cause", &format!("{}", err))?;
                sv.end()
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BindError { ref addr, ref err } => write!(
                f,
                "could not bind TCP sockets to configured address '{}': {}",
                addr, err
            ),
            Error::RunError { ref err } => write!(f, "could not start HTTP server: {}", err),
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
