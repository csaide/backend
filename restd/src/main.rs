// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

extern crate restd;

use std::process;

fn main() {
    let ret = restd::run();
    process::exit(ret);
}
