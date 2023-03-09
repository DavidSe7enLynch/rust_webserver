use log::{debug, error, info};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use thread_pool::ThreadPool;

mod thread_pool;

pub fn lib() -> Result<(), String> {
    let listener =
        TcpListener::bind("127.0.0.1:7878").map_err(|e| format!("start listening err: {e}"))?;

    info!("start listening...");
    let pool = ThreadPool::build(4).map_err(|e| format!("create threadpool err: {e}"))?;

    for stream in listener.incoming() {
        let Ok(stream) = stream else {
            error!("get tcp stream err: {}", stream.unwrap_err());
            continue;
        };
        pool.execute(|| handle_connect(stream)).unwrap_or_else(|e| {
            error!("threadpool executes job err: {e}");
        });
    }
    Ok(())
}

fn handle_connect(mut stream: TcpStream) -> Result<(), String> {
    let request = BufReader::new(&mut stream)
        .lines()
        .next()
        .ok_or("no request line")?
        .map_err(|e| format!("parse line to string err: {e}"))?;
    debug!("received: {:#?}", request);

    let (status, file) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(file).map_err(|e| format!("read file err: {e}"))?;
    let reply = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    );

    stream
        .write_all(reply.as_bytes())
        .map_err(|e| format!("write to tcp stream err: {e}"))?;
    Ok(())
}
