use std::sync::mpsc;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];

    loop {
        // Wait for the client to send us a message.
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            println!("Client closed the connection");
            break;
        }

        stream.write(b"+PONG\r\n").expect("Woo!");
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // handle_connection(stream);
                tx.send(stream).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    for received in rx {
        handle_connection(received)
    }
}
