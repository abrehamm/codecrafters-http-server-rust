use std::{io::Read, io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on port: 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut req = [0u8; 512];
                let _ = stream.read(&mut req);
                let req_str = &String::from_utf8(req.to_vec()).unwrap();
                let req_vec: Vec<&str> = req_str.split("\r\n").collect();
                println!("{}", req_str);
                println!("{:?}", req_vec[0]);
                let first_line: Vec<&str> = req_vec[0].split(' ').collect();
                // let method = first_line[0];
                let path = first_line[1];
                if path == '/'.to_string() {
                    stream
                        .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                        .unwrap();
                } else if path.starts_with("/user-agent") {
                    let mut str_param: &str = "";
                    for line in req_vec.iter() {
                        if line.starts_with("User-Agent") {
                            str_param = line.strip_prefix("User-Agent: ").unwrap();
                            break;
                        }
                    }
                    let mut response = String::new();
                    response.push_str("HTTP/1.1 200 OK\r\n");
                    response.push_str("Content-Type: text/plain\r\n");
                    response.push_str(&format!("Content-Length: {}\r\n\r\n", str_param.len()));
                    response.push_str(str_param);
                    stream.write_all(response.as_bytes()).unwrap();
                } else if path.starts_with("/echo") {
                    let (_, str_param) = path.split_at(6);
                    let mut response = String::new();
                    response.push_str("HTTP/1.1 200 OK\r\n");
                    response.push_str("Content-Type: text/plain\r\n");
                    response.push_str(&format!("Content-Length: {}\r\n\r\n", str_param.len()));
                    response.push_str(str_param);
                    stream.write_all(response.as_bytes()).unwrap();
                } else {
                    stream
                        .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                        .unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
