use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let address = spawn_app();

    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// launch app in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // return the app address to the caller
    format!("http://127.0.0.1:{}", port)
}
