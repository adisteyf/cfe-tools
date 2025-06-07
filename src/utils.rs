use crate::global::*;
use crate::utils::string::FromUtf8Error;
use std::path::{self, Path, PathBuf};
use std::process::{self, Child, Command, Stdio};
use std::{env, string, vec};
use std::{fs, io};

pub fn run_cmd(cmd: &[&str]) -> Child {
    let mut output: Child = match cmd.len() {
        0 => {
            panic!("ERROR,run_cmd: cmd.len() == 0");
        }

        /* if it's the simpliest cmd */
        1 => Command::new(cmd[0])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap(),

        /* otherwise */
        _ => Command::new(cmd[0])
            .args(&cmd[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap(),
    };

    let _res = output.wait();
    return output;
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn cp_scripts(config: Config) {
    let quoted_list: Vec<String> = config
        .scripts
        .list
        .iter()
        .map(|s| String::from(s))
        .collect();

    let exec_path = env::current_exe().unwrap();
    let scripts_path = exec_path.parent().unwrap();

    for folder in quoted_list {
        if fs::metadata(&folder).is_err() {
            let check_script: PathBuf = scripts_path.join(&folder);
            println!("check_script: {:#?}", &check_script);

            if fs::metadata(&check_script).is_ok() {
                copy_dir_all(&check_script, &format!("fe-core/{}", &folder)).unwrap();
            } else {
                println!(
                    "INFO: No '{}' in '.' and in '{:#?}'",
                    &folder, &scripts_path
                );
                panic!("ERROR: Can't find '{}' script", &folder);
            }
            continue;
        }

        copy_dir_all(&folder, &format!("fe-core/{}", &folder)).unwrap();
    }
}

pub fn rm_scripts() {
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

pub fn rm_dir(folder: &str) {
    let path = Path::new(folder);

    /* check if folder doesn't exist */
    if fs::metadata(path).is_err() {
        panic!("ERROR: This dir doesn't exist");
    }

    fs::remove_dir_all(path).expect("ERROR: Can't remove dir.");
}

pub fn mk_dir(folder: &str) {
    let path = Path::new(folder);
    fs::create_dir_all(path).unwrap();
}

pub fn ch_dir(path: &str) {
    env::set_current_dir(path).expect("ERROR: Can't set current dir.");
}

#[allow(unused_variables)]
pub fn gen_fe_includes() -> String {
    let mut headers_vector: Vec<String> = Vec::new();

    for root_entry in fs::read_dir(".").unwrap() {
        let root_entry = root_entry.unwrap();

        if !root_entry.path().is_dir()
            || !root_entry
                .file_name()
                .to_string_lossy()
                .into_owned()
                .starts_with("cfe-")
        {
            continue;
        }

        let root_entry_path = root_entry.path().to_string_lossy().into_owned();
        let incs_path = format!("{}/include", root_entry_path);
        let incs = Path::new(&incs_path);

        for entry in fs::read_dir(incs).unwrap() {
            let entry = entry.unwrap();

            if entry.path().is_dir() {
                continue;
            }

            headers_vector.push(format!(
                "{}",
                entry.file_name().to_string_lossy().into_owned()
            ));
        }
    }

    let mut gen_str = String::from("#ifndef FE_INCLUDES__\n");
    for header in headers_vector {
        let curr_str = format!("#define FE_INCLUDES__\n#include \"{}\"\n", header);
        gen_str.push_str(&curr_str);
    }

    gen_str.push_str("#endif");
    gen_str
}
