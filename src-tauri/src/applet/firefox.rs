// path: the absolute path to the directory that only contains files in .txt format with links separated by lines.
// 
// .txt: each line should start with the url, and all other attributes simply separated by spaces, which will be ignored when loading.
//
// Caution: only for more than a single link. If your file contains only a single link, please use "firefox-url" instead.

// DEV NOTES
// 
// source: https://wiki.mozilla.org/Firefox/CommandLineOptions

#![feature(str_split_whitespace_as_str)]

use std::process::Command;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ffi::OsStr;


pub fn exec(path: &str, base: &PathBuf) {
	//	let mut base = PathBuf::from(path);
	
	let white__space = ' '.to_string();
	let mut file_count = 0;
	let mut command_vector: Vec<Command> = Vec::new();

	let mut full_path = base.clone();
	full_path.push(&path);

	for file in fs::read_dir(&full_path).unwrap() {
		
		if let Ok(file) = file {
			let file_path = file.path();
			match file_path.extension() {
				Some(extension) => {
					if extension == "txt" {
						let mut command = Command::new("firefox");

						// command.arg("--new-window");
						let mut links_string = "".to_string();
						let link_rep = File::open(file_path).unwrap();

						let mut my_vec: Vec<String> = Vec::new();
						
						command_vector.push(Command::new("firefox"));
						let new_window = "-url".to_owned();
						// my_vec.push(new_window);

						command_vector[file_count].arg(&new_window);
						let reader = BufReader::new(&link_rep);
						for line in reader.lines() {
								//println!("{}", line.unwrap());
								let sentence = line.unwrap();
								let first_word = sentence
									.split(" ")
									.next()
									.unwrap_or("");
								links_string.push_str(&white__space);
								links_string.push_str(&first_word);
								let mut word = first_word.to_owned().clone();
								my_vec.push(word);
								command_vector[file_count].arg(&first_word);
								// command.arg(&first_word);
						}
						println!("this is the argument we will pass {}", &links_string);
							
							// command.arg("-new-window");
							// command.arg("--new-tab");
						// Command::new("firefox").arg(links_string).output().expect("done!");
							// command.arg(&links_string);
						println!("{:?}", my_vec);
						command.args(my_vec);
						command_vector[file_count].spawn().expect("process failed to execute");
						file_count = file_count + 1;
						// command.args();
							// links_string.clear();
							// command = Command::new("clear");
							// command.status().expect("process failed to execute");
							// command = Command::new("firefox");
								// let args: Vec<&OsStr> = command.get_args().collect();
								// assert_eq!(args, &["google.com", "bing.com"]);
								// println!("{:?}", args );
							// command = Command::new("firefox");
					} else {
						println!("skpiing file at path ={} due to mismatch extension!", &file_path.display().to_string());
					}
				}
				None => {
					println!("couldn't read file with path ={} due to missing extension ", &file_path.display().to_string());
				}
			}
		}
	}

}