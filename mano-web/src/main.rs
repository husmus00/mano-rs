#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::Router;
    use tower_http::services::ServeDir;
    use tokio::net::TcpListener;

    println!("Mano Machine Web Server");
    println!("========================");
    println!();
    println!("Starting server at http://127.0.0.1:8080");
    println!("Press Ctrl+C to stop");
    println!();

    // Serve static files from dist directory
    let app = Router::new().nest_service("/", ServeDir::new("dist"));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {
    eprintln!("This binary requires the 'ssr' feature to run.");
    eprintln!("Build the WASM app with: trunk build");
    eprintln!("Or run the server with: cargo run --features ssr");
    std::process::exit(1);
}
