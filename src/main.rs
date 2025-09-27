use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct Req {
    method: String,
    path: String,
    version: String,
    headers: Vec<(String, String)>,
    query: HashMap<String, String>,
    body: Option<String>,
}

fn get_query_params(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::<String, String>::new();

    query.split('&').for_each(|val| {
        if let Some((k, v)) = val.split_once("=") {
            params.insert(String::from(k), String::from(v));
        }
    });

    params
}

fn parse_req(raw: &str) -> Req {
    let mut lines = raw.split("\r\n");

    let request_line = lines.next().unwrap();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap().to_string();
    let path = parts.next().unwrap().to_string();
    let query = if let Some((_, q)) = path.clone().split_once('?') {
        get_query_params(q)
    } else {
        HashMap::new()
    };
    println!("{:?}", query);

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
        query,
    }
}

struct Res {
    status_code: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Res {
    fn to_string(&self) -> String {
        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}\r\n", k, v))
            .collect();

        format!(
            "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_text,
            headers,
            self.body.len(),
            self.body
        )
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

    let binding = String::from(http_request);
    let req = parse_req(&binding);

    println!("{:?}", req);

    let res = Res {
        status_code: 200,
        status_text: "OK".to_string(),
        headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        body: format!("{{\"name\": \"{}\"}}", req.path),
    };

    let response_str = res.to_string();

    stream.write_all(response_str.as_bytes()).unwrap();
}
