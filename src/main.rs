use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::os::unix::prelude::ExitStatusExt;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No file specified");
    }
    let file_name = &args[1];
    let file = fs::read(file_name).expect("Something went wrong reading the file");

    let mapped: Vec<u8> = file
        .iter()
        .map(|x| if x == &b'\n' { b'\r' } else { *x })
        .collect();

    if let Ok(mut stream) = TcpStream::connect("10.0.0.47:23") {
        println!("Connected to {}", stream.peer_addr().unwrap());
        stream
            .set_read_timeout(Some(time::Duration::from_millis(100)))
            .unwrap();

        mapped.iter().for_each(|x| {
            stream.write(&[*x]).unwrap();

            if *x == b'\r' {
                let mut response = vec![];
                stream.read(&mut response).unwrap();
                print!("{:?}", String::from_utf8(response));
                thread::sleep(time::Duration::from_millis(300));
            }
        });
    } else {
        println!("Failed to connect");
    }

    Ok(())
}
