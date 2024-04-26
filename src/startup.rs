use crate::routes::{greet, health_check, subscribe};

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // actix-web spins up a worker process for each available core on the machine.
    // Each worker process runs its own copy of HttpServer::new() using the App::new() in the closure as the parameter.
    // So to build  on the app, it needs to implement the Cloneable trait
    // Solution is to use actix-web::Data which wraps the PgConnection in an Atomic Reference
    // Counter which is a pointer to the instance of the PgConnection. This Arc<> is Cloneable.
    let data = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
            .app_data(Data::clone(&data))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
