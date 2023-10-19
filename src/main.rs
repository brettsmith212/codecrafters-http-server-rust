use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("server running");
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let reader = BufReader::new(&_stream);
                let mut lines = reader.lines();
                let first_line = lines.next().unwrap();
                if let Ok(text) = first_line {
                  let mut datas = text.split_whitespace();
                  let _command = datas.next().unwrap();
                  let path = datas.next().unwrap();
                  if path == "/" {
                    _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                  } else {
                    _stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").unwrap();
                  }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
