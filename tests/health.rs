use std::net::TcpListener;

#[tokio::test]
async fn test_health_succeeds() {
    let address = spawn();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to perform health request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn creating_a_contact_succeeds() {
    let address = spawn();
    let client = reqwest::Client::new();

    let data = r#"
        {
            "email": "alice@example.org",
            "first_name": "Alice",
            "last_name": "Awesome"
        }"#;
    let response = client
        .post(&format!("{}/contacts", &address))
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await
        .expect("Failed to add contact.");

    assert_eq!(200, response.status().as_u16());
}

fn spawn() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = calimero::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
