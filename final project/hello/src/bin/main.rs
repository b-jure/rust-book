use std::process;
use std::fs;
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(5)
        .unwrap_or_else(|e| {
            println!("Error while creating ThreadPool: {}", e);
            process::exit(1);
        });

    for stream in listener.incoming() {

        let stream = stream.unwrap();

        pool.execute(|| { 
            handle_connection(stream); 
        });

    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
    
}
