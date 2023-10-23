use std::net::{TcpListener, TcpStream};
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
                  } else if path.starts_with("/echo/") {
                    send_plain_text(&_stream, path);
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

fn send_plain_text(mut stream: &TcpStream, text: &str) {
  let mut data = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n".to_string();
  data.push_str(&format!("Content-Length: {}\r\n\r\n", text.len()));
  data.push_str(text);
  stream.write(data.as_bytes()).unwrap();
}
