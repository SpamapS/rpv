use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

const UPDATE_INTERVAL_SECONDS: u64 = 1;

/// This is the thread that updates status for the user
pub fn status_update(stats: Receiver<usize>) {
    let mut moved: usize = 0;
    let mut done = false;
    let start = Instant::now();
    while !done {
        match stats.recv_timeout(Duration::new(UPDATE_INTERVAL_SECONDS, 0)) {
            Ok(s) => moved += s,
            Err(_) => done = true,
        }
        let since_beginning = start.elapsed().as_secs();
        let rate = if since_beginning != 0 {
            moved as u64 / since_beginning
        } else {
            moved as u64
        };
        eprint!("\r{} -- {}/s    {} since beginning", moved, rate, since_beginning)
    }
}
