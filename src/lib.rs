use env_logger;
use log::{error, info};
use std::{
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
    let buf_reader = BufReader::new(&mut stream);
    let content: Vec<String> = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();
    info!("received: {:#?}", content);

    let reply = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(reply.as_bytes()).unwrap();
}
