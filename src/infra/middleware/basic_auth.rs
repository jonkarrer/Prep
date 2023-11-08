use crate::infra::service::decode_bearer_token;
use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};

// declare name of middleware
pub struct BasicAuth;

// declare wrapper for custom endpoint
pub struct BasicAuthImpl<E>(E);

// impl Middlware trait for middleware struct
impl<E: Endpoint> Middleware<E> for BasicAuth {
    type Output = BasicAuthImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        BasicAuthImpl(ep)
    }
}

// declare name of token to extract
const AUTHORIZATION_TOKEN: &str = "Authorization";

// impl Endpoint trait for custom endpoint
#[poem::async_trait]
impl<E: Endpoint> Endpoint for BasicAuthImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        // get headers from request and find the auth token header
        match req.headers().get(AUTHORIZATION_TOKEN) {
            Some(header_value) => {
                // try to stringify token
                let bearer_token_string = header_value.to_str().map_err(|_| {
                    Error::from_string("Invalid Authorization header", StatusCode::BAD_REQUEST)
                })?;

                // check for correct format
                if !bearer_token_string.starts_with("Bearer ") {
                    return Err(Error::from_string(
                        "Token does not start with Bearer",
                        StatusCode::BAD_REQUEST,
                    ));
                }

                // pull out the token from Bearer email|password
                let encoded_token = &bearer_token_string["Bearer ".len()..];
                let basic_auth = decode_bearer_token(encoded_token)
                    .map_err(|e| Error::from_string(format!("{e}"), StatusCode::BAD_REQUEST))?;

                // AddData to endpoints that use this middleware
                req.extensions_mut().insert(basic_auth);

                // go to next request if all is good
                self.0.call(req).await
            }

            None => Err(Error::from_string(
                "Authorization header not found",
                StatusCode::BAD_REQUEST,
            )),
        }
    }
}
