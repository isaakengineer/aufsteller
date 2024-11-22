extern crate dirs;
use std::path::PathBuf;

use std::process::Command;

pub fn exec(path: &str) {
	println!("Opening with a firefox alternative");
	let quoteMark = r#"""#;
	let mut argumentString =
		// "-url=".to_owned() +
		path;
	// argumentString = argumentString + path;
	// argumentString = argumentString + quoteMark;
	let mut home = dirs::home_dir().unwrap();
    let mut base = PathBuf::from(&home);
    base.push("Anwendungen");
    base.push("zen");
    base.push("zen");
	// let mut command = Command::new("/home/isaak/Apps/zen/zen");
	let mut command = Command::new(base);
	// command.current_dir("~/");
	// command.arg("--app");
	// command.arg(argumentString);
	command.args([argumentString]);
	command.arg("-new-window");
	command.spawn().expect("process failed to execute");
}
