use std::{error::Error, io, process, fs};
use std::path::{Path, PathBuf};
use fs::File;
use serde::{Serialize, Deserialize};

extern crate dirs;
extern crate csv;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pfad {
	pub name: String,
	pub pfad: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Pfadliste {
	pub name: String,
	pub liste: Vec<Pfad>,
}

#[tauri::command]
pub fn profile_lesen() -> Result<Vec<Pfad>, String> {
	let mut result: Vec<Pfad> = Vec::new();
	let mut home = dirs::home_dir().unwrap();
	let mut base = PathBuf::from(&home);
	base.push("Conductor");
	base.push("Profile");
	if base.exists() && base.is_dir() {
		for liste in fs::read_dir(&base).unwrap() {
			if let Ok(liste) = liste {
			let mut pfad_der_liste = liste.path();
				if pfad_der_liste.is_dir() {
					let name = pfad_der_liste.file_stem()
						.and_then(|os_str| os_str.to_str())
						.unwrap_or_else(|| "");
					let profile = Pfad {
						name: name.to_string(),
						pfad: pfad_der_liste.to_str().unwrap().to_string(),
					};
					result.push(profile);
				}
			}
		}
	}
	return Ok(result);
}

#[tauri::command]
pub fn csv_lesen(fach: &str, profile: Option<&str>) -> Result<Vec<Pfadliste>, String> {
// pub fn csv_lesen(fach: &str) {
	let mut result: Vec<Pfadliste> = Vec::new();
	let mut home = dirs::home_dir().unwrap();
	let mut base = PathBuf::from(&home);
	base.push("Conductor");
	match profile {
		Some(profile) => {
			base.push("Profile");
			base.push(profile);
		},
		None => {}
	}
	base.push(fach);
	println!("nach Fach {:?} suchen!", base);
	if base.exists() && base.is_dir() {
		for liste in fs::read_dir(&base).unwrap() {
			if let Ok(liste) = liste {
				let mut pfad_der_liste = liste.path();
				if pfad_der_liste.is_file() {
					match pfad_der_liste.extension() {
						Some(extension) => {
							if extension == "csv" {
								let name = pfad_der_liste.file_stem()
									.and_then(|os_str| os_str.to_str())
									.unwrap_or_else(|| "");
								let mut pfadenliste = Vec::new();
								let datei = File::open(pfad_der_liste.clone());
								match datei {
									Ok(datei) => {
										let mut leser = csv::Reader::from_reader(datei);
										for result in leser.deserialize() {
											match result {
												Ok(result) => {
													let record: Pfad = result;
													pfadenliste.push(record);
												},
												Err(e) => {
													let m = format!("csv Datei hat falsche Formatierung! Siehe: {:?}", e);
													return Err(m);
												}
											}
										}
									},
									Err(e) => {
										let m = format!("könnte die Datei ({:?}) nicht lesen. Siehe: {:}", pfad_der_liste, e);
										return Err(m);
									}
								}
								let mut pfadliste = Pfadliste {
									name: name.to_string(),
									liste: pfadenliste
								};
								result.push(pfadliste);
							}
						},
						None => continue,  // Skip files with no extension
					}
				}
			}
		}
		Ok(result)
	} else {
		let m = format!("Der eigegebene Pfad ({:?}) ist nicht vorhanden.", base.to_str());
		return Err(m);
	}
}
