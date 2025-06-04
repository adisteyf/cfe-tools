#![allow(unused_imports)]
use std::{fs, process::Command, str};
mod build_core;
mod global;
mod help;
mod install_core;
use crate::build_core::build_core;
use crate::global::Config;
use crate::global::read_config;
use crate::help::{print_help, print_version};
use crate::install_core::install_core;

fn main() {
    let cmd: String;
    match std::env::args().nth(1) {
        Some(v) => cmd = v,
        None => {
            println!("ERROR: unexpected argument for command field.");
            return;
        }
    }

    match cmd.as_str() {
        "install-core" => install_core(),
        "build-core" => build_core(),
        "--help" => print_help(),
        "-h" => print_help(),
        "--version" => print_version(),
        "-v" => print_version(),
        _ => println!("ERROR: unexpected argument for command field."),
    }
}
