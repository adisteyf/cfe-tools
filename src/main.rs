use std::{str,fs,process::Command};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    cmds: Cmds,
}

#[derive(Debug, Deserialize)]
struct Cmds {
    shell_cmd: Vec<String>,
    install_core: String,
}

fn main() {
    let cmd: String;
    match std::env::args().nth(1) {
        Some(v) => cmd = v,
        None    => { println!("ERROR: unexpected argument for command field."); return; },
    }

    match cmd.as_str() {
        "install-core" => install_core(),
        _              => println!("ERROR: unexpected argument for command field."),
    }
}

fn read_config() -> Config {
    let file = fs::read_to_string("config.toml")
        .expect("ERROR: Can't read config.toml");
    let config: Config = toml::from_str(&file).unwrap();
    return config;
}

fn install_core() {
    let config: Config = read_config();
    println!("removing old fe-core...");
    let _rm_old_core = Command::new("rm")
        .args(["-rf", "fe-core"])
        .output()
        .expect("cmd err");
    println!("installing fe-core...");

    if config.cmds.shell_cmd.len() == 0 || config.cmds.install_core.len() == 0 {
        panic!("ERROR: no commands in config.toml");
    }

    if config.cmds.shell_cmd.len() == 1 {
        let _install_core = Command::new(config.cmds.shell_cmd[0].clone())
            .args([config.cmds.install_core])
            .output()
            .expect("cmd err");
    }

    else {
        let mut args = config.cmds.shell_cmd.clone();
        args.remove(0);
        args.push(config.cmds.install_core);

        let _install_core = Command::new(config.cmds.shell_cmd[0].clone())
            .args(args)
            .output()
            .expect("cmd err");
    }

    println!("successfully installed core");
}
