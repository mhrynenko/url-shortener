mod handlers;
mod main;

use hyper::body::{Body, Bytes};
use hyper::{Request, Response};
use tokio::net::TcpListener;
use http_body_util::{combinators::BoxBody, BodyExt};
use axum::{
    http::StatusCode, Router,
    routing::get,
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
            .route("/", get(handlers::reverse_body::reverse_body))
    }
}