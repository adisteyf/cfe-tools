use crate::global::*;
use crate::utils::string::FromUtf8Error;
use std::path::{self, Path};
use std::process::{self, Command};
use std::{fs, io};
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

pub fn cp_scripts(config: Config) {
    let quoted_list: Vec<String> = config
        .scripts
        .list
        .iter()
        .map(|s| format!("'{}'", s))
        .collect();
    let folders_list = quoted_list.join(" ");

    println!("copying new fe-core scripts...");
    let _cp_new_scripts = run_cmd(&vec!["cp", "-r", folders_list.as_str(), "fe-core"]);
}

pub fn rm_scripts() {
    /*let _rm_old_scripts = Command::new("bash")
    .args(["-c", "rm -rf fe-core/cfe-*"])
    .output()
    .expect("cmd err");*/

    let fe_core = Path::new("fe-core");
    for entry in fs::read_dir(fe_core).expect("ERROR,rm_scripts: Can't read fe-core folder.") {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir()
            || !path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("cfe-")
        {
            continue;
        }

        fs::remove_dir_all(&path).unwrap();
        println!("INFO: Deleted dir: {:?}", path);
    }
}
