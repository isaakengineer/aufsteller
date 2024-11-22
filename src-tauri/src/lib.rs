#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::io::Write;
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};

use toml;

extern crate dirs;

mod applet;
mod konfig;

use applet::{anwendung_oeffnen};

use konfig::{ csv_lesen, profile_lesen};

#[derive(Serialize, Deserialize, Debug)]
struct Notiz {
	pfad: String,
	name: String,
	inhalt: String,
}
#[derive(Serialize, Deserialize)]
struct WorkspaceIndex {
  desktop: Vec<Desktop>,
}
#[derive(Serialize, Deserialize)]
struct Desktop {
  window: Vec<Window>,
}
#[derive(Serialize, Deserialize)]
struct Window {
  applet: String,
  path: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn projects_list() -> Vec<String> {
	let mut home = dirs::home_dir().unwrap();
	let mut base = PathBuf::from(&home);
	base.push("Workspaces");
	let mut projects = Vec::new();
	for item in fs::read_dir(&base).unwrap() {
		if let Ok(item) = item {
			let mut item_path = item.path();
			let meta = fs::metadata(&item_path).unwrap();
			if meta.file_type().is_dir() {
				let project_name = item.file_name().into_string().unwrap();
				projects.push(project_name);
			}
		}
	}
	println!("{:?}", projects);
	return projects;
}
#[tauri::command]
fn app_start(name: String) {
	println!("running the app here");
	applet::webapp::exec(&name);
}
#[tauri::command]
fn project_on(name: String) -> WorkspaceIndex {
	println!("let's do some greetings here");
	let mut home = dirs::home_dir().unwrap();
	let mut base = PathBuf::from(&home);
	base.push("Workspaces");
	base.push(name);
	let workspace_path = base.clone();
	base.push("index.toml");

	let contents = match fs::read_to_string(&base) {
		Ok(c) => c,
		Err(e) => {
		  println!("reading workspace index from path = `{}` failed!", base.display().to_string());
		  panic!("Error loading local config file. {}", e);
		}
	};
	println!("{:?}", &contents);

	let index: WorkspaceIndex = match toml::from_str(&contents) {
		Ok(d) => d,
		Err(e) => {
		println!("File to process config from = `{}`", base.display().to_string());
		panic!("Error reading from file. {}", e);
		}
	};
	for desktop in &index.desktop {
		let mut new_command = Command::new("wmctrl");
		new_command.arg("-d");
		let mut result = new_command.output().expect("process failed");
		let stdout = String::from_utf8(result.stdout).unwrap();
		println!("the workspaces\n{}", stdout);
		let main_vec = stdout.lines()
			.map(|s| s.trim().split("  ").map(String::from).collect::<Vec<String>>())
			.collect::<Vec<Vec<String>>>();
		let mut workspaces_count = main_vec.len();
		new_command.arg("-s").arg( (workspaces_count - 1).to_string() );
		new_command.status().expect("process failed to execute");

		let directory = "directory".to_owned();
		let code = "code".to_owned();

		for window in &desktop.window {
			let applet_name = window.applet.as_str();
			println!("applet name: {}", &applet_name);
			match applet_name {
				"directory" => {
					println!("open VS Code");
					applet::directory::exec(&window.path);
				},
				"code" => {
					println!("open nautilus");
					applet::code::exec(&window.path);
				},
				"firefox" => {
					println!("web browser going crazy");
					applet::firefox::exec(&window.path, &workspace_path);
				},
				"md" => {
					println!("gnome editor");
					applet::md::exec(&window.path);
				},
				"open" => {
					println!("document viewr");
					applet::open::exec(&window.path);
				},
				_ => {
					println!("don't know what to do!");
				}
			}
		}
	}
	return index;
}

#[tauri::command]
fn dashboard_config_load(name: String) -> Result<String, String>{
	let mut home = dirs::home_dir().unwrap();
	let mut base = PathBuf::from(&home);
	base.push("Conductor");
	base.push(name);
	println!("base: {}", base.display().to_string());
	match fs::read_to_string(&base) {
		Ok(c) => return Ok(c),
		Err(e) => {
			return Err("reading workspace index from path = `{}` failed!".to_string())
		}
	}
}

#[tauri::command]
fn album_init() -> Result<Vec<String>, String> {
	let home = dirs::home_dir().unwrap();
	let mut base = PathBuf::from(&home);
	base.push("Conductor");
	base.push("Album");
	let mut bildern = Vec::new();
	match fs::read_dir(&base) {
		Ok(entries) => {
            for entry in entries {
				match entry {
					Ok(entry) => {
						let path = entry.path();
						if path.is_file() && path.extension().map(|ext| ext == "jpg").unwrap_or(false) {
							bildern.push(path.to_str().unwrap().to_string());
						}
					}
					Err(e) => {
						println!("Error reading entry: {}", e);
						// Err("könnte nicht lesen".to_string())
					}
				}
			}
		},
		Err(e) => {
			println!("Error reading entry: {}", e);
			// Err("könnte nicht lesen".to_string())
		}
	}
	Ok(bildern)
}
#[tauri::command]
fn notizen_init() -> Result<Vec<Notiz>, String> {
	let home = dirs::home_dir().unwrap();
	let mut base = PathBuf::from(&home);
	base.push("Conductor");
	base.push("Notizen");
	let mut notizen:Vec<Notiz> = Vec::new();
	match fs::read_dir(&base) {
		Ok(entries) => {
            for entry in entries {
				match entry {
					Ok(entry) => {
						let path = entry.path();
						if path.is_file() && path.extension().map(|ext| ext == "md").unwrap_or(false) {
							let mut notiz:Notiz;
							match fs::read_to_string(path.clone()) {
								Ok(inhalt) => {
									notiz = Notiz {
										pfad: path.to_str().unwrap_or_else(|| "".as_ref()).to_string(),
										name: path.file_name().unwrap_or_else(|| "".as_ref()).to_string_lossy().into_owned(),
										inhalt: inhalt,
									};
								},
								Err(e) => {
									notiz = Notiz {
										pfad: path.to_str().unwrap_or_else(|| "".as_ref()).to_string(),
										name: path.file_name().unwrap_or_else(|| "".as_ref()).to_string_lossy().into_owned(),
										inhalt: "".to_string(),
									};
								}
							}
							notizen.push(notiz);
						}
					}
					Err(e) => {
						println!("Error reading entry: {}", e);
						// Err("könnte nicht lesen".to_string())
					}
				}
			}
		},
		Err(e) => {
			println!("Error reading entry: {}", e);
			// Err("könnte nicht lesen".to_string())
		}
	}
	Ok(notizen)
}


#[tauri::command]
fn app_open(exec: String, path: String) {
	println!("exec: {}", exec);
	println!("path: {}", path);
	let mut command = Command::new(exec);
	command.arg(path);
	command.spawn().expect("failed to open directory with supplied app.");
}

#[tauri::command]
fn switch_display_input(id: u32, password: String) {
	let mut new_command = Command::new("sudo");
	new_command.arg("-S");
	new_command.arg("ddcutil");
	new_command.arg("setvcp");
	new_command.arg("0x60");
	if id == 1 {
		new_command.arg("0x03");
	} else {
		new_command.arg("0x04");
	}
	// let mut result = new_command.status().unwrap();
	// result.stdin().as_mut().unwrap().write_str()
	// let mut result = new_command.output().expect("process failed");
	// let stdout = String::from_utf8(result.stdout).unwrap();
	// println!("the workspaces\n{}", stdout);
	let mut child = new_command.stdin(Stdio::piped())
		.stdout(Stdio::inherit())
		.spawn()
		.unwrap();

	if let Some(mut stdin) = child.stdin.take() {
		stdin.write_all(password.as_bytes()).unwrap();
		stdin.write_all(b"\n").unwrap();
	}

	let status = child.wait().unwrap();
	println!("status: {}", status);
}

#[tauri::command]
fn greet(name: &str) {
	let mut new_command = Command::new("wmctrl");
	//.arg("-c")
	//.arg("echo hello")
	//.output()
	//.expect("failed to execute process");
	new_command.arg("-d");
	let mut result = new_command.output().expect("process failed");
	//new_command.status()
	let stdout = String::from_utf8(result.stdout).unwrap();
	println!("the workspaces\n{}", stdout);
	let main_vec = stdout.lines()
		.map(|s| s.trim().split("  ").map(String::from).collect::<Vec<String>>())
		.collect::<Vec<Vec<String>>>();
	let mut workspaces_count = main_vec.len();
	print!("main vector\n{:?} \n", main_vec);
	//let hello = new_command.stdout;
	//println!("{}", hello);
	//println!("{}", new_command.status);
	println!("number of workspaces {:?} \n", workspaces_count.to_string() );
	new_command.arg("-s").arg( (workspaces_count - 1).to_string() );
	new_command.status().expect("process failed to execute");

	let mut nautilus_command = Command::new("nautilus");
	nautilus_command.arg("~/Downloads");
	nautilus_command.status().expect("process failed to execute");
	println!();
	let res = format!("{}, ! You've been greeted from Rust!", name);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			album_init,
			notizen_init,

			dashboard_config_load,
			app_open,
			app_start,
			projects_list,
			project_on,
			switch_display_input,
			// project_read
			// project_open

			// notes_list
			// note_read
			greet, // sample function shipped by Tauri

			anwendung_oeffnen,
			csv_lesen,
			profile_lesen,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
