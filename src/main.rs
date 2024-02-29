mod api;

#[tokio::main]
async fn main() {
    let api = api::Api {addr: String::from("127.0.0.1:3000") };
    api.run().await;
}
