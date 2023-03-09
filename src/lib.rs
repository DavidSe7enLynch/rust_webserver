use log::{debug, info};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use thread_pool::ThreadPool;

mod thread_pool;

pub fn lib() {
    env_logger::Builder::new().parse_filters("debug").init();
    let listener = TcpListener::bind("127.0.0.1:7878").expect("start listening fail");
    info!("start listening...");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.expect("acquire tcp stream fail");
        pool.execute(|| handle_connect(stream));
    }
}

fn handle_connect(mut stream: TcpStream) {
    let request = BufReader::new(&mut stream)
        .lines()
        .next()
        .expect("no request line")
        .expect("parse line fail");
    debug!("received: {:#?}", request);

    let (status, file) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(file).expect("read file fail");
    let reply = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );
    stream
        .write_all(reply.as_bytes())
        .expect("write to tcp stream fail");
}
