use std::{
    io::{BufRead, BufReader, Error, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line: Vec<&str> = http_request[0].split(" ").collect();

    match request_line[0] {
        "GET" => {
            if request_line[1] == "/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
            } else if request_line[1].starts_with("/echo/") {
                let response = request_line[1].replace("/echo/", "");
                stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
            } else {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        }
        _ => {
            println!("Unknown method: {}", request_line[0]);
        }
    }

    Ok(())
}
