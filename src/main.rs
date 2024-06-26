use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = &configuration.database.connection_string();
    let db_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres DB");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // if problems then return the Error, otherwise call .await
    run(listener, db_pool)?.await
}
