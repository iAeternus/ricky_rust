use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use anyhow::Result;
use http_server::ThreadPool;

// fn main() -> Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
//     let pool = ThreadPool::new(4);

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         pool.execute(|| {
//             let _ = handle_connection(stream);
//         });
//     }

//     Ok(())
// }

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match router(&request_line) {
        Some((s, f)) => (s, f),
        None => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn router(request: &str) -> Option<(&str, &str)> {
    if request.starts_with("GET / HTTP/1.1") {
        Some(("HTTP/1.1 200 OK", "hello.html"))
    } else if request.starts_with("GET /sleep HTTP/1.1") {
        thread::sleep(Duration::from_secs(5));
        Some(("HTTP/1.1 200 OK", "hello.html"))
    } else {
        None
    }
}
