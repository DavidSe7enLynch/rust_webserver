use env_logger;
use log::{debug, error, info};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub fn lib() {
    env_logger::Builder::new().parse_filters("debug").init();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    info!("start listening");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connect(stream);
    }
}

fn handle_connect(mut stream: TcpStream) {
    let request: Vec<_> = BufReader::new(&mut stream)
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();
    debug!("received: {:#?}", request);

    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let reply = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );
    stream.write_all(reply.as_bytes()).unwrap();
}
