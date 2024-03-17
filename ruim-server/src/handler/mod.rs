use std::collections::HashMap;

use axum::{body::Body, http::{HeaderName, Response}, response::IntoResponse};
use serde_json::json;



pub mod chat;
pub mod user;


#[derive(Debug)]
pub struct ApiError {
    msg: String,
    code : axum::http::StatusCode,
}

impl ApiError {
    pub fn msg(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(mut self, code: axum::http::StatusCode) -> Self {
        self.code = code;
        self
    }
}

impl From<anyhow::Error> for ApiError{
    fn from(value: anyhow::Error) -> Self {
        Self {
            msg: value.to_string(),
            code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError{
    fn into_response(self) -> axum::response::Response {
        let body = axum::Json(json!({
            "error": self.msg,
        }));

        (self.code, body).into_response()
    }
}

pub struct GenericResponse {
    pub status: axum::http::StatusCode,
    pub body: Body,
    pub headers: HashMap<HeaderName, String>,
}

impl IntoResponse for GenericResponse {
    fn into_response(self) -> Response<Body> {
        let mut res = Response::new(self.body);
        *res.status_mut() = self.status;
        for (k, v) in self.headers {
            res.headers_mut().insert(k, v.parse().unwrap());
        }
        res
    }
}

impl Default for GenericResponse {
    fn default() -> Self {
        Self {
            status: axum::http::StatusCode::OK,
            body: Body::empty(),
            headers: HashMap::new(),
        }
    }
}

impl GenericResponse {
    pub fn code (mut self, code: axum::http::StatusCode) -> Self {
        self.status = code;
        self
    }

    pub fn body (mut self, body: Body) -> Self {
        self.body = body;
        self
    }

    pub fn header(mut self, key: HeaderName, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn json<T: serde::Serialize>(self, data: T) -> Self {
        let body = serde_json::to_vec(&data).unwrap();
        self.body(Body::from(body)).header(HeaderName::from_static("content-type"), "application/json".to_string())
    }

    pub fn text(self, text: String) -> Self {
        self.body(Body::from(text)).header(HeaderName::from_static("content-type"), "text/plain".to_string())
    }

    pub fn msg(self, msg: &str) -> Self {
        self.json(json!({ "msg": msg }))
    }
}