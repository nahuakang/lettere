//! tests/health_check.rs

// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml.
//
// You can inspect what code gets generated using `cargo expand --test health_check`.

#[actix_rt::test]
async fn health_check_test() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = lettere::run().expect("failed to bind address");
    let _ = tokio::spawn(server);
}
