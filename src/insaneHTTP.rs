use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

mod lib;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    let pool = lib::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request*: {}", String::from_utf8_lossy(&buffer[..]));

    let get_index = b"GET / HTTP/1.1\r\n";
    let get_css = b"GET /bootstrap.min.css HTTP/1.1";
    let get_js = b"GET /insane-british-anagram.js";
    let get_wasm = b"GET /insane-british-anagram_bg.wasm";
    let get_dict = b"GET /british-english-insane.txt";

    if buffer.starts_with(get_index) {
        let contents = fs::read_to_string("www/index.html").unwrap();

        let h1 = "HTTP/1.0 200 OK\r\n".to_string();
        let h2 = "Server: InsaneHTTP/0.0.1 Rust\r\n";
        let h3 = "Date: Sun, 11 Aug 2019 13:57:08 GMT\r\n";
        let h4 = "Content-type: text/html\r\n";
        let h5 = "Content-Length: 5317\r\n";
        let h6 = "Last-Modified: Sun, 11 Aug 2019 13:43:57 GMT\r\n";

        let response = format!("{}{}{}{}{}{}\r\n\r\n{}", h1, h2, h3, h4, h5, h6, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(get_css) {
        println!("\nCSS requested.");
        let contents = fs::read_to_string("www/bootstrap.min.css").unwrap();

        let h1 = "HTTP/1.0 200 OK\r\n".to_string();
        let h2 = "Server: InsaneHTTP/0.0.1 Rust\r\n";
        let h3 = "Date: Sun, 11 Aug 2019 13:57:08 GMT\r\n";
        let h4 = "Content-type: text/css\r\n";
        let h5 = "Content-Length: 5317\r\n";
        let h6 = "Last-Modified: Sun, 11 Aug 2019 13:43:57 GMT\r\n";

        let response = format!("{}{}{}{}{}{}\r\n\r\n{}", h1, h2, h3, h4, h5, h6, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(get_js) {
        println!("\nJS requested.");
        let contents = fs::read_to_string("www/insane-british-anagram.js").unwrap();

        let h1 = "HTTP/1.0 200 OK\r\n".to_string();
        let h2 = "Server: InsaneHTTP/0.0.1 Rust\r\n";
        let h3 = "Date: Sun, 11 Aug 2019 13:57:08 GMT\r\n";
        let h4 = "Content-type: application/javascript\r\n";
        let h5 = "Content-Length: 4472\r\n";
        let h6 = "Last-Modified: Sun, 11 Aug 2019 13:43:57 GMT\r\n";

        let response = format!("{}{}{}{}{}{}\r\n\r\n{}", h1, h2, h3, h4, h5, h6, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(get_wasm) {
        println!("\nWASM requested.");
        let contents = fs::read("www/insane-british-anagram_bg.wasm").unwrap();

        let h1 = "HTTP/1.0 200 OK\r\n".to_string();
        let h2 = "Server: InsaneHTTP/0.0.1 Rust\r\n";
        let h3 = "Date: Sun, 11 Aug 2019 13:57:08 GMT\r\n";
        let h4 = "Content-type: application/wasm\r\n";
        let h5 = "Content-Length: 56576\r\n";
        let h6 = "Last-Modified: Sun, 11 Aug 2019 13:43:57 GMT\r\n";

        let response = format!("{}{}{}{}{}{}\r\n", h1, h2, h3, h4, h5, h6);

        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(get_dict) {
        println!("\nDictionary requested.");
        let contents = fs::read("www/british-english-insane.txt").unwrap();

        let h1 = "HTTP/1.0 200 OK\r\n".to_string();
        let h2 = "Server: InsaneHTTP/0.0.1 Rust\r\n";
        let h3 = "Date: Sun, 11 Aug 2019 13:57:08 GMT\r\n";
        let h4 = "Content-type: text/html\r\n";
        let h5 = "Content-Length: 6875495\r\n";
        let h6 = "Last-Modified: Sun, 11 Aug 2019 13:43:57 GMT\r\n";

        let response = format!("{}{}{}{}{}{}\r\n", h1, h2, h3, h4, h5, h6);

        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("www/404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
