use rudvent::build_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the shared build function
    let router = build_router("Alice".to_string());

    // Do the serving on its own
    let port = 8000;
    println!("Server listening on {}", port);
    axum::Server::bind(&format!("127.0.0.1:{}", port).parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
