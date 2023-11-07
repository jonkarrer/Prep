use crate::infra::authentication::auth;
use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};

// declare name of middleware
pub struct SessionValidation;

// declare wrapper for custom endpoint
pub struct SessionValidationImpl<E>(E);

// impl Middlware trait for middleware struct
impl<E: Endpoint> Middleware<E> for SessionValidation {
    type Output = SessionValidationImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        SessionValidationImpl(ep)
    }
}

// declare name of token to extract
const SESSION_COOKIE_KEY: &str = "session_id";

pub struct UserId(String);

// impl Endpoint trait for custom endpoint
#[poem::async_trait]
impl<E: Endpoint> Endpoint for SessionValidationImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let cookies = req.cookie();
        // get headers from request and find the auth token header
        match cookies.get(SESSION_COOKIE_KEY) {
            Some(cookie) => {
                let session_token = cookie.value::<&str>().map_err(|_| {
                    Error::from_string(
                        "Invalid session_id value in cookie",
                        StatusCode::BAD_REQUEST,
                    )
                })?;

                let mut auth = auth().await;

                let user_id = auth
                    .validate_session(session_token)
                    .await
                    .map_err(|_| Error::from_string("Session expired", StatusCode::BAD_GATEWAY))?;

                // AddData to endpoints that use this middleware
                req.extensions_mut().insert(UserId(user_id));

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
