// underlying usage: XDG-Open (https://linux.die.net/man/1/xdg-open)

use std::process::Command;

pub fn exec(path: &str) {
	let mut command = Command::new("xdg-open");
	// command.arg("-n");
	command.arg(path);
	command.spawn().expect("process failed to execute");
}