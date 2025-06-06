use crate::utils::string::FromUtf8Error;
use std::process::{self, Command};
use std::{string, vec};

pub fn run_cmd(cmd: &[&str]) -> String {
    let output: String = match cmd.len() {
        0 => {
            panic!("ERROR,run_cmd: cmd.len() == 0");
        }

        /* if it's the simpliest cmd */
        1 => String::from_utf8(
            Command::new(cmd[0])
                .output()
                .expect("ERROR,run_cmd: Can't run the command.")
                .stdout,
        )
        .expect("ERROR,run_cmd: Can't get stdout."),

        /* otherwise */
        _ => String::from_utf8(
            Command::new(cmd[0])
                .args(&cmd[1..])
                .output()
                .expect("ERROR,run_cmd: Can't run the command.")
                .stdout,
        )
        .expect("ERROR,run_cmd: Can't get stdout."),
    };

    return output;
}
