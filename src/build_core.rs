use crate::global::*;
use crate::utils::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

fn build_code() {
    let core_build_path = match env::current_dir() {
        Ok(p) => {
            let mut p = p;
            p.push("fe-core");
            p.push("build");
            p
        }
        Err(e) => panic!("ERROR,build_code: Can't get current path: {}", e),
    };

    /*if !core_build_path.is_dir() {
        panic!("ERROR: Can't get fe-core folder.");
    }*/

    println!(
        "Path to fe-core build folder: {}",
        core_build_path.to_str().unwrap()
    );
    match fs::create_dir_all(&core_build_path) {
        Ok(_) => println!("Created 'build' dir"),
        Err(e) => panic!("ERROR: Can't create the folder: {}", e),
    }

    /* configure using cmake */
    println!("Configuring using CMake...");
    env::set_current_dir(&core_build_path).expect("ERROR: Can't set current dir.");
    let cmake_out = run_cmd(&vec!["cmake", ".."]);
    println!("{}", cmake_out);

    /* build using make */
    println!("Building using Make...");
    let make_out = run_cmd(&vec!["make"]);
    println!("{}", make_out);
}

pub fn build_core() {
    let config: Config = read_config();

    println!("removing old fe-core scripts...");
    let _rm_old_scripts = Command::new("bash")
        .args(["-c", "rm -rf fe-core/cfe-*"])
        .output()
        .expect("cmd err");

    let quoted_list: Vec<String> = config
        .scripts
        .list
        .iter()
        .map(|s| format!("'{}'", s))
        .collect();
    let folders_list = quoted_list.join(" ");

    println!("copying new fe-core scripts...");
    let _cp_new_scripts = Command::new("bash")
        .args(["-c", &format!("cp -r {} fe-core", folders_list.as_str())])
        .output()
        .expect("cmd err");

    build_code();
    println!("done!");
}
