use std::io::{Read, Write};
use std::fs::{OpenOptions, File};
use std::net::{TcpListener, TcpStream};


fn create_file(message_bytes: Vec<u8>, file_path: &str) {
    let mut lines: Vec<Vec<u8>> = Vec::new();
    let mut current_line: Vec<u8> = Vec::new();

    for b in message_bytes {
        if b != 10_u8 {
            current_line.push(b);
        } else {
            lines.push(current_line.clone());
            current_line.clear();
        }
    }

    let mut message_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    for line in lines {
        if let Err(e) = writeln!(message_file, "{}", std::str::from_utf8(&line).unwrap()) {
            eprintln!("Error. Could not write to file {}", e);
        }
    }
}

fn receive_input(mut stream: TcpStream) -> Vec<u8> {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());

    let mut message_bytes: Vec<u8> = Vec::new();

    let mut buf = [0; 512];
    let mut bytes_read = 2;
    let mut final_byte: u8 = 1;

    while bytes_read > 1 && final_byte != 0 {
        bytes_read = stream.read(&mut buf).unwrap();
        message_bytes.extend(&buf[..bytes_read-1]);
        final_byte = buf[bytes_read-1];
    }
    message_bytes
}

pub fn start(dest_path: &str) {
    let mut message_bytes: Vec<u8>;
    let mut _file: File = match File::create(dest_path) {
        Ok(_f) => _f,
        Err(_e) => panic!("Error. Could not create file {}", dest_path),
    };

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Error. Could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(_e) => panic!("Error. Failed to process stream"),
            Ok(stream) => {
                message_bytes = receive_input(stream);
                create_file(message_bytes, dest_path);
            }
        };
    }
}
