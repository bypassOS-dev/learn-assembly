use std::{
    io::{BufRead, BufReader, Write}, 
    net::{TcpListener, TcpStream},
};
use std::fs;

fn main() {
    println!("Start!");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("listening...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conection(stream);
    }
}
fn handle_conection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "bypassOS.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n {}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
