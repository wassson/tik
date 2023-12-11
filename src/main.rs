use std::fs::File;
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
    let home_dir = dirs::home_dir().unwrap();
    let new_dir = home_dir.join(".tik");
    let tik_dir = new_dir.as_path();

    let stdout = File::create(tik_dir.join("tik.out")).unwrap();
    let stderr = File::create(tik_dir.join("tik.err")).unwrap();

    match fs::create_dir_all(tik_dir) {
        Err(e) => println!("Error creating directory: {:?}", e.kind()),
        _ => (),
    }

    let daemonize = Daemonize::new()
        .pid_file(new_dir.join("tik.pid")) 
        .chown_pid_file(true)
        .user("tik")
        .group("tik")
        .working_directory(new_dir.to_str().unwrap())
        .umask(0o777) 
        .stdout(stdout) 
        .stderr(stderr); 

    match daemonize.start() {
        Ok(v) => {
            println!("{:?}", v);
            println!("Success, daemonized");
            run();
        },
        Err(e) => eprintln!("Error, {}", e),
    }
}
