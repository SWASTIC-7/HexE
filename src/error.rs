use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn log_error(message: &str) {
    let now = SystemTime::now();
    let log_entry = format!("{:?}: ERROR - {}\n", now, message);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

pub fn log_info(message: &str) {
    let now = SystemTime::now();
    let log_entry = format!("{:?}: INFO - {}\n", now, message);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("simulator.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}

pub fn log_warning(message: &str) {
    let now = SystemTime::now();
    let log_entry = format!("{:?}: WARNING - {}\n", now, message);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("simulator.log")
    {
        let _ = file.write_all(log_entry.as_bytes());
    }
}
