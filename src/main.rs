mod api;
mod data;
mod config;

#[tokio::main]
async fn main() {
    let cfg = config::Config::new().unwrap();

    let api = api::Api {addr: String::from(format!("{}:{}", cfg.listener.host, cfg.listener.port)) };
    api.run().await;
}
