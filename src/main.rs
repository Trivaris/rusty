use core::panic;
use::std::io::{Read,Write};
use::std::net::{TcpListener,TcpStream,IpAddr,Ipv4Addr};
use::std::thread::spawn;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    let request_result = stream.read(&mut buffer);
    match request_result {
        Ok(_) => {},
        Err(err) => panic!("Failed to read from TCP Stream: {}", err.to_string()),
    };

    let decoded_request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request, {}", decoded_request);

    let response = "Hello, Client!".as_bytes();
    let response_result = stream.write(response);
    match response_result {
        Ok(_) => {},
        Err(err) => panic!("Failed to read to TCP Stream: {}", err.to_string()),
    }
}

fn main() {
    const ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    const PORT: u16 = 8080;

    let listener = match TcpListener::bind((ADDR, PORT)) {
        Ok(it) => it,
        Err(err) => panic!("Failed to bind to lister: {}", err.to_string()), 
    };

    println!("Server listening on {}:{}",ADDR.to_string(),PORT);

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                spawn(|| handle_client(stream));
            },
            Err(err) => eprint!("Failed to establish connection: {}", err),
        }
    }
}