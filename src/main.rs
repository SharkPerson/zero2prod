use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =
        TcpListener::bind("127.0.0.1:8000").expect("Could not bind to the address provided");
    // if problems then return the Error, otherwise call .await
    run(listener)?.await
}
