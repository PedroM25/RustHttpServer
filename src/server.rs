use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}...", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection accepted");
                    println!("[DEB] Connection accepted from {}", addr.to_string());
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(size) => {
                            println!("[DEB] Read {} bytes from TCP stream", size);
                            println!("Incoming message from connection: {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(err) => {
                            println!("Error while reading from connection: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to estrablish a connection: {}", err);
                }
            }
        }
    }
}
