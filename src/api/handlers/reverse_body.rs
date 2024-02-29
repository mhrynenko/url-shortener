use http_body_util::{Empty, Full};
use hyper::body::{Body, Bytes};
use hyper::{Request, Response};
use http_body_util::{combinators::BoxBody, BodyExt};
use axum::{
    http::StatusCode,
};

pub async fn reverse_body(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let upper_bound = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if upper_bound > 1024 * 64 {
        let mut resp = Response::new(full("Too big body to handle"));
        *resp.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;
        return Ok(resp);
    }

    let body = req.collect().await?.to_bytes();

    let reversed_body = body.iter().rev().cloned().collect::<Vec<u8>>();

    Ok(Response::new(full(reversed_body)))
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
