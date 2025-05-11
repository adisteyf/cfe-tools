use crate::global::*;
use std::process::Command;

pub fn install_core() {
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
