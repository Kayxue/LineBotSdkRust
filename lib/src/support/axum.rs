use axum::extract::{FromRequest, Request};
use hyper::StatusCode;

#[derive(Debug)]
pub struct Signature {
    pub key: String,
}

impl<S> FromRequest<S> for Signature
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(x_line_signature) = req.headers().get("x-line-signature") {
            if let Ok(key) = x_line_signature.to_str() {
                Ok(Signature {
                    key: key.to_owned(),
                })
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
