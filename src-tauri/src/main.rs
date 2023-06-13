#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;

use toml;

extern crate dirs;

mod applet;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_start,
            projects_list,
            project_on,
            // project_read
            // project_open

            // notes_list
            // note_read
            greet, // sample function shipped by Tauri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
