
use std::process::Command;

pub fn exec(path: &str) {
	let mut command = Command::new("gnome-text-editor");
	command.arg("-n");
	command.arg(path);
	command.spawn().expect("process failed to execute");
}