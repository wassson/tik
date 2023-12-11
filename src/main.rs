use std::fs::{File, OpenOptions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::{process, fs};
use std::{thread, time};
use daemonize::Daemonize;

fn run() {
    let one_second = time::Duration::from_secs(1);
    loop {
        println!("pid: {}", process::id());
        thread::sleep(one_second);
    }
}

fn main() {
    // TODO: Check if process is already running
    let home_dir = dirs::home_dir().unwrap();
    let tik_dir = home_dir.join(".tik");
    
    match fs::create_dir_all(&tik_dir) {
        Err(e) => println!("Error creating directory: {:?}", e.kind()),
        _ => (),
    }

    let file = tik_dir.join("tik.pid");
    let _f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file);

    let permissions = Permissions::from_mode(0o777);
    let _ = std::fs::set_permissions(&file, permissions);

    let stdout = File::create(tik_dir.join("tik.out")).unwrap();
    let stderr = File::create(tik_dir.join("tik.err")).unwrap();

    // TODO: Revisit to privileged_action()
    let daemonize = Daemonize::new()
        .pid_file(tik_dir.join("tik.pid")) 
        .chown_pid_file(true)
        .working_directory(tik_dir.to_str().unwrap())
        .umask(0o777) 
        .stdout(stdout) 
        .stderr(stderr) 
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(v) => {
            println!("{:?}", v);
            println!("Success, daemonized");
            run();
        },
        Err(e) => eprintln!("Error, {}", e),
    }
}
