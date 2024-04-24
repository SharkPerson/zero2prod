use std::net::TcpListener;
// Can use cargo expand --test health_check
#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let address: String = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute target");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length()); // check the body is empty
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
