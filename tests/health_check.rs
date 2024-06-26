use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{configuration::DatabaseSettings, startup::run};

use sqlx::{Connection, Executor, PgConnection, PgPool};
use zero2prod::configuration::get_configuration;

#[allow(dead_code)]

struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Can use cargo expand --test health_check
#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute target");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length()); // check the body is empty
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // POST HTTP request at /subscriptions
    // Return 200 status code
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let valid_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(valid_body)
        .send()
        .await
        .expect("failed to execute target");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both the name and email"),
    ];

    for (invalid_body, desc) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute target");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with a bad request when the payload was {}",
            desc
        );
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read config");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&configuration.database).await;
    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);
    TestApp { address, db_pool }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
