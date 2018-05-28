use std::net::{TcpListener, TcpStream};

pub fn listen_and_serve(port: i32, handle_func: &Fn(TcpStream)) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).unwrap();

    for s in listener.incoming() {
        let s = match s {
            Ok(stream) => handle_func(stream),
            Err(error) => panic!("You fool! {}", error),
        };
    }
}
