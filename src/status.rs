extern crate number_prefix;
use self::number_prefix::{decimal_prefix,Standalone,Prefixed, Amounts};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::fmt::Display;
use std::thread::sleep;

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
pub fn status_update(moved: Arc<AtomicUsize>, done: Arc<AtomicBool>) {
    let start = Instant::now();
    let interval = Duration::from_secs(UPDATE_INTERVAL_SECONDS);
    while !done.load(Ordering::Relaxed) {
        let elapsed = start.elapsed();
        let moved = moved.load(Ordering::Relaxed);
        let rate: f64 = if elapsed.as_secs() != 0 {
            moved as f64 / (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9)
        } else {
            moved as f64
        };
        let rate = to_pretty(rate, 4);
        let now_moved = to_pretty(moved as f64, 4);
        eprint!("\r{} -- {}    {:?} since beginning", now_moved, rate, elapsed);
        sleep(interval);
    }
}
