use std::{fs, thread};
use std::time::Duration;

use super::http::{Request, RequestError, RequestMethod, Response, StatusCode};

#[derive(Debug)]
pub struct WebHandler {
    path: String,
}

impl WebHandler {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                fs::read_to_string(path).ok()
            }
            Err(_) => None
        }
    }

    pub fn handle_request(&self, req: &Request) -> Response {
        match req.method() {
            RequestMethod::Post => todo!(),
            RequestMethod::Get => match req.path() {
                "/" => Response::ok(self.read_file("index.html")),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(5));
                    Response::ok(self.read_file("index.html"))
                },
                path => match self.read_file(path) {
                    Some(content) => Response::ok(Some(content)),
                    None => Response::not_found(self.read_file("not_found.html")),
                }
            },
            RequestMethod::Update => todo!(),
            RequestMethod::Delete => todo!(),
            RequestMethod::Head => todo!(),
            RequestMethod::Connect => todo!(),
            RequestMethod::Patch => todo!(),
            RequestMethod::Trace => todo!(),
            RequestMethod::Options => todo!(),
        }
    }

    pub fn handle_err_request(&self, e: &RequestError) -> Response {
        eprintln!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

