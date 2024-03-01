use axum::{
    extract::Json, response::Response,
};
use serde::{Serialize, Deserialize};
use sha3::{Digest, Sha3_256};
use hex;
use crate::api::responses;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortLink {
    link: String,
}

pub async fn short_link(payload: Json<ShortLink>) -> Response{
    let res = ShortLink{link:sha256(payload.link.as_bytes())};
    println!("{:?}", responses::success(serde_json::to_vec(&res).unwrap()));

    responses::success(serde_json::to_vec(&res).unwrap())
}

fn sha256(payload: &[u8]) -> String{
    let mut hasher = Sha3_256::new();
    hasher.update(payload);
    hex::encode(hasher.finalize())
}

