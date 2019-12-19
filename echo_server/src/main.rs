use std::io;
// use std::io::Write;
// use std::io::Read;
// https://doc.rust-lang.org/std/io/prelude/index.html
// If not "prelude", we have to use io::Write and io::Read
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread::spawn;
use hex_slice::AsHex;

/// Accept connections forever, spawning a thread for each one.
fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        // Wait for a client to connect.
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from {}", addr);
        // Spawn a thread to handle this client.
        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            // Echo everything we receive from `stream` back to it.
            // io::copy(&mut stream, &mut write_stream).expect("error in client thread: ");
            let mut buf = [0u8; 512];
            let mut written = 0;
            loop {
                let len = match stream.read(&mut buf) {
                    Ok(0) => {
                        println!("connection closed, written: {}", written);
                        return Ok(written)
                    },
                    Ok(len) => len,
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        println!("connection error {}", e);
                        return Err(e)
                    },
                };
                println!("Read and write: {:X}, Length: {}", &buf[..len].as_hex(), len);
                write_stream.write_all(&buf[..len])?;
                written += len as u64;
            }
        });
    }
}

fn main() {
    echo_main("127.0.0.1:17007").expect("error: ");
}
