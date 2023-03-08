use std::net::{TcpListener, TcpStream};



fn lib() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connect(&stream);
    }
}

fn handle_connect(stream: &TcpStream) {
    
}