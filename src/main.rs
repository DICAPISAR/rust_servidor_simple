use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    // Iniciar servidor
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(&address).unwrap();
    print!("Servidor iniciado en {}", &address);

    // Escuchar por conexiones
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conection(stream);
    }
}

// Manejar conexiones

fn handle_conection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Stream recibido");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    // Responder al cliente
    send_to_client(stream);
}

fn send_to_client(mut stream: TcpStream) {
    let contents = fs::read_to_string("index.html").unwrap();
    
    let response = format! (
        "HTTP/1.1 200 OK\r\n Content-Length: {}\r\n\r\n{}",  
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}