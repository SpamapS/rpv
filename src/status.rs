extern crate number_prefix;
use self::number_prefix::{decimal_prefix,Standalone,Prefixed, Amounts};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};
use std::fmt::Display;

const UPDATE_INTERVAL_SECONDS: u64 = 1;

fn to_pretty<T>(bytes: T, places: usize) -> String
where T: Amounts,
      T: Display {
    match decimal_prefix(bytes) {
        Standalone(bytes) => format!("{:9}B", bytes),
        Prefixed(prefix, bytes) => format!("{:9.*}{}", places, bytes, prefix),
    }
}

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
        let elapsed = start.elapsed();
        let rate: f64 = if elapsed.as_secs() != 0 {
            moved as f64 / (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9)
        } else {
            moved as f64
        };
        let rate = to_pretty(rate, 4);
        let now_moved = to_pretty(moved as f64, 4);
        eprint!("\r{} -- {}    {:?} since beginning", now_moved, rate, elapsed)
    }
}
