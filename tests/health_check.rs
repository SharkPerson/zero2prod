use std::net::TcpListener;

// Can use cargo expand --test health_check
#[tokio::test]
async fn health_check_works() {
    let address: String = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
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
    let address = spawn_app();
    let client = reqwest::Client::new();

    let valid_body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(valid_body)
        .send()
        .await
        .expect("failed to execute target");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both the name and email"),
    ];

    for (invalid_body, desc) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
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

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
