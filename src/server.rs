use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => {
                            println!("Stream read error: {}", e)
                        }
                    }
                }
                Err(e) => println!("Connection error: {}", e),
            }
        }
    }
}
