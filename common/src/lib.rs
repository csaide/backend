// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

#[macro_use]
extern crate slog;
#[macro_use]
extern crate slog_derive;
extern crate erased_serde;
extern crate jsonwebtoken as jwt;
extern crate structopt;
pub mod jwt;
pub mod log;
