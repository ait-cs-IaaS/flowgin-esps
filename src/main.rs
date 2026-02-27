use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Unable to listen for tokio");
    axum::serve(listener, app)
        .await
        .expect("Unable to serve with axum");
}

async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}
