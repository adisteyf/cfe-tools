use crate::global::*;
use std::process::Command;

pub fn build_core() {
    let config: Config = read_config();

    println!("removing old fe-core scripts...");
    let _rm_old_scripts = Command::new("bash")
        .args(["-c", "rm -rf fe-core/cfe-*"])
        .output()
        .expect("cmd err");

    let quoted_list: Vec<String> = config.scripts.list
        .iter()
        .map(|s| format!("'{}'", s))
        .collect();
    let folders_list = quoted_list.join(" ");

    println!("copying new fe-core scripts...");
    let _cp_new_scripts = Command::new("bash")
        .args(["-c", &format!("cp -r {} fe-core", folders_list.as_str())])
        .output()
        .expect("cmd err");

    println!("done!");
}
