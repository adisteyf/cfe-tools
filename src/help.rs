const _HELP: &str = "usage: cfe-tools [-v | --version] [-h | --help]

commands:
	install-core    Install FilesEngine core
	build-core      Build FilesEngine core with scripts
	build-remove    Remove FilesEngine build";
const _VERSION: &str = "1.5.2";

pub fn print_help() {
    println!("{}", _HELP);
}

pub fn print_version() {
    println!("cfe-tools version: {}", _VERSION);
}
