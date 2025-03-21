extern crate dirs;
use std::path::PathBuf;

use std::process::Command;

#[cfg(target_os = "windows")]
pub fn exec(input_pth: &str) {
    use std::process::Command;

    Command::new("cmd")
        .args(&["/C", "start", input_pth])
        .spawn()
        .expect("Failed to execute command");
}

#[cfg(target_os = "macos")]
pub fn exec(input_pth: &str) {
    use std::process::Command;

    Command::new("open")
        .arg("-n")
        .arg(input_pth)
        .spawn()
        .expect("Failed to execute command");
}

#[cfg(target_os = "linux")]
pub fn exec(input_pth: &str) {
    use std::process::Command;

    let apps = ["librewolf", "zen", "firefox"];

    for app in apps.iter() {
        let app_available = Command::new("which")
            .arg(app)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if app_available {
            Command::new(app)
                .arg("--new-window")
                .arg(input_pth)
                .spawn()
                .expect("Failed to execute command");
            return; // Exit the function after launching the app
        }
    }

    // If none of the apps are available, fallback to xdg-open
    Command::new("xdg-open")
        .arg(input_pth)
        .spawn()
        .expect("Failed to execute command");
}
