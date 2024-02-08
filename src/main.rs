use anyhow::Result;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().merge(hello_router());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("error in listener");
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener,app).await.unwrap();
    Ok(())
}

fn hello_router() -> Router {
    Router::new().route("/", get(hello_handler))
}

async fn hello_handler() -> impl IntoResponse {
    Html(format!(
        "
        <!doctype html>
        <html>
            <head>
                <title>hello</title>
            </head>
            <body>
                <h1>hello K</h1>
            </body>
        </html>
        "
    ))
}
