extern crate os_type;
extern crate sysinfo;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::Command;

use sysinfo::{DiskExt, System, SystemExt};

use crate::cli::run;

mod requirements;
mod cli;
mod utils;
mod create_image;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    set_default_log_level();
    run()
}

fn set_default_log_level() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter_level(log::LevelFilter::Info);
    builder.init();
}