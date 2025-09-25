use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct Req {
    method: String,
    path: String,
    version: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

fn parse_req(raw: &str) -> Req {
    let mut lines = raw.split("\r\n");

    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();
    let version = parts.next().unwrap().to_string();

    let mut headers = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(": ") {
            headers.push((k.to_string(), v.to_string()));
        }
    }

    let body = lines.collect::<Vec<_>>().join("\r\n");
    let body = if body.is_empty() { None } else { Some(body) };

    Req {
        method,
        path,
        version,
        headers,
        body,
    }
}

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

    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{:?}", parse_req(&String::from(http_request)));
}
