use std::thread;
use std::time;
use chrono::Utc;
use chrono::SecondsFormat;

fn main() {
    let mut count = 0;
    loop {
        println!("[{}] count = {}", Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true), count);
        thread::sleep(time::Duration::new(1, 0));
        count += 1;
    }
}
