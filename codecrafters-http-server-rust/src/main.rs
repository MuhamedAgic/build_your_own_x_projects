use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;


fn collect_request(stream: &TcpStream) -> Vec<String>{
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    return http_request;
}

fn handle_echo_request(path_chunks: &Vec<String>) -> String {
    path_chunks[2..].concat() // just print everything behind /echo for now
}

fn get_response(http_request: &Vec<String>) -> String {
    if http_request.get(0).is_none() {
        return "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }

    // first line example: GET / HTTP/1.1
    let request_line = http_request.get(0).unwrap();
    let parts: Vec<&str> = request_line.split(' ').collect();
    if parts.get(1).is_none() {
        return "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }

    let mut response = String::from("");
    let path = parts.get(1).unwrap();
    let path_chunks: Vec<String> = path
        .split('/')
        .map(|s| s.to_string())
        .collect(); // e.g. /echo/abc -> ["echo", "abc"]

    println!("{:?}", path_chunks);
    if path_chunks.len() == 1 {
        return "HTTP/1.1 200 Ok\r\n\r\n".to_string();
    } else if path_chunks.contains(&"echo".to_string()) {
        let response_content = handle_echo_request(&path_chunks);
        response = String::from(format!("HTTP/1.1 200 {response_content}\r\n\r\n"));
    } else {
        return "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }

    return response.to_string();
}

fn handle_connection(mut stream: TcpStream) {
    let http_request: Vec<String> = collect_request(&stream);
    println!("Received request: \n\n{}", http_request.join("\n"));

    let response: String = get_response(&http_request);
    println!("Sending response: {}\n\n", response);
    stream.write_all(response.as_bytes()).unwrap();
}


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
