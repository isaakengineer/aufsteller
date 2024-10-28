use std::process::Command;

pub fn exec(path: &str) {
	println!("executing app path");
	let quoteMark = r#"""#;
	let mut argumentString = "--app=".to_owned() + path;
	// argumentString = argumentString + path;
	// argumentString = argumentString + quoteMark;
	let mut command = Command::new("chromium-browser");
	// command.current_dir("~/");

	// command.arg("--app");
	command.args([argumentString]);
	// command.arg(argumentString);
	// command.arg("--new-window");
	command.spawn().expect("process failed to execute");
}
