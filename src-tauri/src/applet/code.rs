use std::process::Command;

pub fn exec(path: &str) {
	let mut command = Command::new("code");
	command.current_dir(path);
	command.arg("-n");
	command.arg(".");
	command.spawn().expect("process failed to execute");
}