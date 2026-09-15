use axum::Router;

fn create_app() -> Router {
    Router::new()
        .route("/", get(health_check));
}

fn main() {
    let app = create_app();
}
