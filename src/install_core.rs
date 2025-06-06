use crate::global::*;
use crate::utils::run_cmd;
use std::process::Command;

pub fn install_core() {
    let config: Config = read_config();
    println!("removing old fe-core...");
    let _rm_old_core = run_cmd(&vec!["rm", "-rf", "fe-core"]);
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
