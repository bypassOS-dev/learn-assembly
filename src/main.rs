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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|resault| resault.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let contents = fs::read_to_string("bypassOS.html").unwrap();

    let response = format!(
        "HTTP/1.1/ 200 OK\r\nContent-Length: {}\r\n\r\n {}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
