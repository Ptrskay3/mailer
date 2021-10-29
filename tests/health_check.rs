use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to parse address.");
    let port = listener.local_addr().unwrap().port();

    let server = learning_actix::run(listener).expect("Failed to boot server.");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}