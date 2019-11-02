// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Super usings
use super::{handler, level};

#[derive(Debug, Clone)]
pub struct Config {
    pub handler: handler::Handler,
    pub level: level::Level,
    pub path: String,
}
