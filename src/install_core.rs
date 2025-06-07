use crate::global::*;
use crate::utils::{rm_dir, run_cmd};
use std::fs;
use std::process::Command;

pub fn install_core() {
    let config: Config = read_config();

    if fs::metadata("fe-core").is_ok() {
        println!("removing old fe-core...");
        let _rm_old_core = rm_dir("fe-core");
    }

    println!("installing fe-core...");
    if config.cmds.install_core.len() == 0 {
        panic!("ERROR: no commands in config.toml");
    }

    let args: Vec<&str> = config
        .cmds
        .install_core
        .iter()
        .map(|s| s.as_str())
        .collect();
    let _install_core = run_cmd(&args);

    println!("successfully installed core");
}
