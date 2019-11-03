// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(
        long = "rest-port",
        short = "p",
        help = "The port to listen on for incoming HTTP requests.",
        default_value = "8080",
        takes_value = true
    )]
    pub port: u16,

    #[structopt(
        long = "rest-addr",
        short = "a",
        help = "The address to listen on for incoming HTTP requests.",
        default_value = "0.0.0.0",
        takes_value = true
    )]
    pub addr: String,
}
