use std::collections::HashMap;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode, body::Body,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ApiError {
    status: u16,
    title: String,
    description: Option<String>,
}

pub struct ApiResponse {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub fn internal_error() -> Response{
    ApiResponse {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        headers: HashMap::new(),
        body: serde_json::to_vec(&ApiError{
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            title: String::from("Internal Server Error"),
            description: None,
        }).unwrap(),
    }.with_json_header().into_response()
}

pub fn bad_request() -> Response{
    ApiResponse {
        status: StatusCode::BAD_REQUEST,
        headers: HashMap::new(),
        body: serde_json::to_vec(&ApiError{
            status: StatusCode::BAD_REQUEST.as_u16(),
            title: String::from("Bad Request"),
            description: None,
        }).unwrap(),
    }.with_json_header().into_response()
}

pub fn not_found() -> Response{
    ApiResponse {
        status: StatusCode::NOT_FOUND,
        headers: HashMap::new(),
        body: serde_json::to_vec(&ApiError{
            status: StatusCode::NOT_FOUND.as_u16(),
            title: String::from("Not Found"),
            description: None,
        }).unwrap(),
    }.with_json_header().into_response()
}

pub fn success(body: Vec<u8>) -> Response {
    ApiResponse{
        status: StatusCode::OK,
        headers: HashMap::new(),
        body,
    }.with_json_header().into_response()
}

impl ApiResponse {
    fn with_json_header(mut self) -> Self {
        self.headers.insert(String::from("content-type"), String::from("application/vnd.api+json"));
        self
    }

    fn with_status(mut self, status_code: StatusCode) -> Self {
        self.status = status_code;
        self
    }

    fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let mut response = Response::builder().status(self.status);
        if !self.headers.is_empty() {
            for (key, val) in self.headers.iter() {
                response = response.header(key, val)
            }
        }
        response.body(Body::from(self.body)).unwrap()
    }
}
