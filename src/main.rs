use axum::{
    routing::get,
    Router,
    Json,
    http::StatusCode,
    response::Html
};

async fn index() -> &'static str { "home" }
async fn about() -> &'static str { "about" }
async fn list_users() -> &'static str { "List Users" }
async fn create_users() -> &'static str { "Create Users" }
async fn get_user() -> &'static str { "Get Users" }
async fn serve_file() -> &'static str { "Serve File" }

async fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn json() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "message" : "hello" }))
}

async fn json() -> Html<&'static str> {
    Html("<h1>hoi</h1>")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/users", get(list_users).post(create_users));

    let user_routes = Router::new()
        .route("/users/{id}", get(get_user))
        .route("/files/{*path}", get(serve_file));
        

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
