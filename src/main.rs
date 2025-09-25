use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");

    match listener {
        Ok(listener) => {
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                handle_connection(stream);
            }
        }
        Err(error) => println!("{:?}", error),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).unwrap();

    println!("{:#?}", http_request);
}
