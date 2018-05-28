mod lib;

use lib::listen_and_serve;
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

    listen_and_serve(80, "hello_world")
}

fn hello_world(s: TcpStream) {
    println!("eek");
}
