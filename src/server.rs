#![allow(dead_code)]

use std::{net::TcpListener, str::FromStr};
use std::io::Read;
use std::net::IpAddr;
use std::sync::Arc;
use rust_multi_thread_server::ThreadPool;

use crate::http::{Request, RequestError};
use crate::web_handler::WebHandler;

#[derive(Debug)]
pub struct Server {
    address: String,
    port: u16,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        Server { address, port }
    }

    pub fn full_address(addr: &str) -> Result<Self, RequestError> {
        Server::from_str(addr)
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    pub fn run(&self, handler: Arc<WebHandler>) {
        let addr = self.address();
        let listener = TcpListener::bind(addr).unwrap();

        let pool = ThreadPool::new(5);

        for stream in listener.incoming() {
            let handler = Arc::clone(&handler);
            pool.execute(move || {
                match stream {
                    Ok(mut stream) => {
                        println!("Connection established!");

                        let mut buffer = [0; 1024];

                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(req) => handler.handle_request(&req),
                                    Err(e) => handler.handle_err_request(&e),
                                };

                                if let Err(e) = response.send(&mut stream) {
                                    eprintln!("Failed to send response: {:?}", e);
                                }
                            }
                            Err(e) => println!("Failed to read from connection : {:?}", e)
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to establish a connection: {:?}", e)
                    }
                }
            })
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: String::from("127.0.0.1"),
            port: 8080,
        }
    }
}

impl FromStr for Server {
    type Err = RequestError;

    // Exp: 127.0.0.1:8081
    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = addr.trim().split(':').collect();

        if v.len() != 2 {
            return Err(RequestError::InvalidIpAddress);
        }

        let address = v.first().unwrap().to_string();
        let port: u16 = v
            .last()
            .unwrap()
            .parse()
            .or(Err(RequestError::InvalidIpAddress))?;

        let ip_addr = IpAddr::from_str(&address)?;
        let address = ip_addr.to_string();

        Ok(Server { address, port })
    }
}