use std::{str,fs,process::Command};
mod global;
mod install_core;
mod build_core;
use crate::global::Config;
use crate::global::read_config;
use crate::install_core::install_core;
use crate::build_core::build_core;

fn main() {
    let cmd: String;
    match std::env::args().nth(1) {
        Some(v) => cmd = v,
        None    => { println!("ERROR: unexpected argument for command field."); return; },
    }

    match cmd.as_str() {
        "install-core" => install_core(),
        "build-core"   => build_core(),
        _              => println!("ERROR: unexpected argument for command field."),
    }
}

