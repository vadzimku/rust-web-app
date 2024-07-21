#[tokio::main]
async fn main() {
    let app = rust_web_app::build_router();

    let _ = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_or_else(
            |err| panic!("Error while creating tcp listener: {}", err),
            |listener| axum::serve(listener, app),
        )
        .await
        .map_err(|err| panic!("Error while starting web app: {}", err));
}
