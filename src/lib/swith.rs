#[tauri::command]
fn switch_display_input(id: u32) {
    let mut command = Command::new("sudo");
    command.arg("ddcutil")
        .arg("setvcp")
        .arg("0x60");
    if id == 1 {
        command.arg("0x01");
    } else {
        command.arg("0x04");
    }
    let mut child = command.stdin(Stdio::piped())
        // .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute");
    if let Some(mut stdin) = child.stdin.take() {
        let stdin_handle = io::stdin();
        let mut stdin_lock = stdin_handle.lock();
        loop {
            let line = stdin_lock
                .lines()
                .next()
                .expect("Failed to read from stdin");
            let line = line.trim();
            if line == "quit" {
                break;
            }
            stdin.write_all("Se@G+Romy\n".as_bytes())
                .expect("Failed to write to stdin");
            stdin.write_all(b"\n").expect("Failed to write to stdin");
        }
    }

    // let mut stdin = child.stdin.take().expect("failed to opten stdin");
    // std::thread::spawn(move || {
        
    // });
    let output = child.wait().expect("failed to read out");
    // let mut result = new_command.spawn().unwrap();
    // let mut p2 = result.stdin.take().unwrap();
    // // p2.write_all("your_root_password_here\n".as_bytes()).unwrap();
    // p2.write_all("Se@G+Romy\n".as_bytes()).unwrap();
    // p2.flush().unwrap();
    // p2.wait().unwrap();

    // let mut dest = Command::new("wc")
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .unwrap();
    // let _source = Command::new("ls")
    //     .stdout(dest.stdin.take().unwrap())
    //     .spawn()
    //     .unwrap();
    // let dest_output = dest.wait_with_output().unwrap();
}