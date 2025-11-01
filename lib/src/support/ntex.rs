use ntex::{
    http::Payload,
    web::{
        ErrorRenderer, FromRequest, HttpRequest,
        error::{self, ErrorBadRequest},
    },
};

#[derive(Debug)]
pub struct Signature {
    pub key: String,
}

impl<Err: ErrorRenderer> FromRequest<Err> for Signature {
    type Error = error::Error;

    async fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Result<Self, Self::Error> {
        if let Some(x_line_signature) = req.headers().get("x-line-signature") {
            if let Ok(key) = x_line_signature.to_str() {
                Ok(Signature {
                    key: key.to_string(),
                })
            } else {
                Err(ErrorBadRequest("x-line-signature can't be parsed").into())
            }
        } else {
            Err(ErrorBadRequest("x-line-signature is missing").into())
        }
    }
}
