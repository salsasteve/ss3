use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

fn arr(a: [u8; 4]) -> u32 {
    let mut b: u32 = 0;
    for i in 0..4 {
        b = b + (a[i] as u32) * (256 as u32).pow(3 - i as u32);
    }
    b
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            print!("Recieved a  {}", String::from_utf8_lossy(&buffer));
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}

