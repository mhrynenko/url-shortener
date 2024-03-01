mod handlers;
mod responses;

use handlers::{
    short_link::short_link,
};
use tokio::net::TcpListener;
use axum::{
    Router,
    routing::post,
};


pub struct Api {
    pub addr: String,
}

impl Api {
    pub async fn run(&self) {
        println!("start api server on {}", self.addr);

        let listener = TcpListener::bind(self.addr.clone()).await.unwrap();
        axum::serve(listener, self.router()).await.unwrap();
    }

    fn router(&self) -> Router {
        Router::new()
            .route("/", post(short_link))
    }
}