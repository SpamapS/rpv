///    This program is free software: you can redistribute it and/or modify
///    it under the terms of the GNU General Public License as published by
///    the Free Software Foundation, either version 3 of the License, or
///    (at your option) any later version.
///
///    This program is distributed in the hope that it will be useful,
///    but WITHOUT ANY WARRANTY; without even the implied warranty of
///    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
///    GNU General Public License for more details.
///
///    You should have received a copy of the GNU General Public License
///    along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[macro_use] extern crate log;
extern crate env_logger;
extern crate bytes;
extern crate rpv;
use rpv::status::status_update;
use std::io::{stdin, stdout, Read, Write};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use bytes::BytesMut;

fn main() {
    env_logger::init();
    debug!("spawning status thread");
    let moved = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(AtomicBool::new(false));
    let t_moved = moved.clone();
    let t_done = done.clone();
    let status_thread = thread::spawn(move || {
        status_update(t_moved, t_done)
    });
    debug!("spawned status thread");
    loop {
        let mut buffer = BytesMut::with_capacity(8192);
        // Must fill it with 0's so it has a length
        buffer.extend([0; 8192].iter());
        let bytes_read = stdin().read(&mut buffer).unwrap();
        debug!("I read {} bytes", bytes_read);
        if bytes_read == 0 {
            done.store(true, Ordering::Relaxed);
            status_thread.join().unwrap();
            return
        }
        while buffer.len() > 0 {
            debug!("Sending...");
            let sent = stdout().write(&buffer).unwrap();
            debug!("Sent {} bytes", sent);
            moved.fetch_add(sent, Ordering::Relaxed);
            buffer.split_to(sent);
        }
    }
}
