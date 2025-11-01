use axum::{extract::FromRequestParts, http::request::Parts};
use hyper::StatusCode;

#[derive(Debug)]
pub struct Signature(pub String);

impl<S> FromRequestParts<S> for Signature
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(x_line_signature) = parts.headers.get("x-line-signature") {
            if let Ok(key) = x_line_signature.to_str() {
                Ok(Signature(key.to_owned()))
            } else {
                Err((
                    StatusCode::BAD_REQUEST,
                    "x-line-signature can't be parsed",
                ))
            }
        } else {
            Err((StatusCode::BAD_REQUEST, "x-line-signature is missing"))
        }
    }
}
