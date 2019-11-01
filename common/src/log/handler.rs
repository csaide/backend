// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Super usings
use super::error::{Error, Result};

// Standard usings
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Handler {
    File,
    Stdout,
}

impl FromStr for Handler {
    type Err = Error;

    fn from_str(t: &str) -> Result<Handler> {
        match t {
            "file" => Ok(Handler::File),
            "stdout" => Ok(Handler::Stdout),
            _ => Err(Error::HandlerMissing {
                handler: t.to_owned(),
            }),
        }
    }
}
