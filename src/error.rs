use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn log_error(message: &str) {
    let now = SystemTime::now();
    let log_entry = format!("{:?}: {}\n", now, message);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")
        .expect("Unable to open error.log");
    file.write_all(log_entry.as_bytes()).expect("Write failed");
}