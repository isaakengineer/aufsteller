use std::path::{PathBuf};
use std::process::{Command};

pub mod code;
pub mod directory;
pub mod firefox;
pub mod md;
pub mod open;
pub mod webapp;

pub fn pfad_vorhanden(pfad: String) -> Result<bool, String>{
	let datei = PathBuf::from(&pfad);
	if datei.exists() {
		Ok(true)
	} else {
		let m = format!("Dateipfad ist nicht vorhanden.");
		Err(m)
	}
}

#[tauri::command]
pub fn anwendung_oeffnen(exec: String, path: String) -> Result<bool, String> {
	match pfad_vorhanden(path.clone()) {
		Ok(_) => {
			let mut command = Command::new(exec);
				command.arg(path);
				command.spawn().expect("failed to open directory with supplied app.");
			Ok(true)
		},
		Err(e) => {
			Err(e)
		}
	}
}
