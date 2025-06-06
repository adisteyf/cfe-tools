use crate::global::*;
use crate::utils::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

fn build_code() {
    mk_dir("fe-core/build");

    /* configure using cmake */
    println!("Configuring using CMake...");
    ch_dir("fe-core/build");
    let _cmake_out = run_cmd(&vec!["cmake", ".."]);

    /* build using make */
    println!("Building using Make...");
    let _make_out = run_cmd(&vec!["make"]);
}

pub fn build_core() {
    let config: Config = read_config();

    println!("removing old fe-core scripts...");
    rm_scripts();
    println!("pasting new scripts...");
    cp_scripts(config);
    build_code();
    println!("done!");
}
