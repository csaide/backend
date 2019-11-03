// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Super usings
use super::{handler, level};

use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Config {
    #[structopt(
        long = "log-handler",
        short = "t",
        env = "LOG_HANDLER",
        help = "The logging handler to use.",
        default_value = "stdout",
        possible_values = &["stdout", "file"],
        takes_value = true
    )]
    pub handler: handler::Handler,

    #[structopt(
        long = "log-level",
        short = "l",
        env = "LOG_LEVEL",
        help = "The logging level to use.",
        default_value = "info",
        possible_values = &["critical", "error", "warn", "info", "debug"],
        takes_value = true
    )]
    pub level: level::Level,

    #[structopt(
        long = "log-file",
        short = "f",
        env = "LOG_FILE",
        help = "The log file to write to if the 'file' log handler is used.",
        default_value = "",
        required_if("log_handler", "file"),
        takes_value = true
    )]
    pub path: String,
}
