use std::{
    io::{prelude::*, BufReader},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
};

fn main() {
    let socket_path = "/tmp/rust_server.sock";

    if Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path).unwrap();
    }

    let listener = UnixListener::bind(socket_path).unwrap();

    println!("Listening on Unix socket: {}", socket_path);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: UnixStream) {
    let buffer_reader = BufReader::new(&stream);
    let request = buffer_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!("Request: {:?}", request);

    let (status, contents) = match request[0].as_str() {
        "GET /say-hello HTTP/1.1" => {
            let status = "HTTP/1.1 200 OK";
            let contents = r#"{"message": "Hello, World!"}"#;

            (status, contents)
        }
        "GET /say-goodbye HTTP/1.1" => {
            let status = "HTTP/1.1 200 OK";
            let contents = r#"{"message": "Goodbye, World!"}"#;

            (status, contents)
        }
        _ => {
            let status = "HTTP/1.1 404 NOT FOUND";
            let contents = r#"{"message": "Not Found"}"#;

            (status, contents)
        }
    };

    let length = contents.len();

    let response = format!(
        "{status}\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
