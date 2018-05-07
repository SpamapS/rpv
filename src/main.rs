#[macro_use] extern crate log;
extern crate env_logger;
extern crate bytes;
extern crate rpv;
use rpv::status::status_update;
use std::io::{stdin, stdout, Read, Write};
use std::sync::mpsc::channel;
use std::thread;
use bytes::BytesMut;

fn main() {
    /// Read from stdin (or file)
    /// write to stdout (or file)
    /// ocassionally print how fast things are moving (and if we can, how much of input we've read)
    env_logger::init();
    debug!("spawning status thread");
    let (tx, rx) = channel();
    let status_thread = thread::spawn(|| { status_update(rx) });
    debug!("spawned status thread");
    loop {
        let mut buffer = BytesMut::with_capacity(8192);
        // Must fill it with 0's so it has a length
        buffer.extend([0; 8192].iter());
        let bytes_read = stdin().read(&mut buffer).unwrap();
        debug!("I read {} bytes", bytes_read);
        if bytes_read == 0 {
            drop(tx);
            status_thread.join().unwrap();
            return
        }
        while buffer.len() > 0 {
            debug!("Sending...");
            let sent = stdout().write(&buffer).unwrap();
            debug!("Sent {} bytes", sent);
            tx.send(sent).unwrap();
            buffer.split_to(sent);
        }
    }
}
