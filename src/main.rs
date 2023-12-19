use std::env;
use std::sync::Arc;

use server::Server;

use crate::http::RequestError;
use crate::web_handler::WebHandler;

pub mod server;
pub mod http;
pub mod web_handler;

fn main() -> Result<(), RequestError> {
    let server = Server::full_address("127.0.0.1:8080")?;

    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    println!("{}", public_path);

    let handler = Arc::new(WebHandler::new(public_path));

    server.run(handler);

    Ok(())
}
