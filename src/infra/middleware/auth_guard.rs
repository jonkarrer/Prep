use crate::{domain::entity::UserId, infra::authentication::auth};
use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};

// declare name of middleware
pub struct AuthGuard;

// declare wrapper for custom endpoint
pub struct AuthGuardImpl<E>(E);

// impl Middlware trait for middleware struct
impl<E: Endpoint> Middleware<E> for AuthGuard {
    type Output = AuthGuardImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthGuardImpl(ep)
    }
}

// declare name of token to extract
const SESSION_COOKIE_KEY: &str = "session_id";

// impl Endpoint trait for custom endpoint
#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthGuardImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        match req.headers().get("Cookie") {
            Some(header_value) => {
                // turn cookie header value into a str
                let cookies_str = header_value.to_str().map_err(|_| {
                    Error::from_string("Invalid Cookie header", StatusCode::BAD_REQUEST)
                })?;

                // find session_id in cookie str
                let session_str = cookies_str
                    .split(";")
                    .find(|x| x.contains(SESSION_COOKIE_KEY))
                    .ok_or(Error::from_string(
                        "Session id not found in cookie",
                        StatusCode::BAD_REQUEST,
                    ))?;

                // parse out the token
                let session_token = &session_str["session_id=".len()..];

                // validate session
                let mut auth = auth().await;
                let user_id = auth.validate_session(&session_token).await.map_err(|e| {
                    Error::from_string(format!("{e}"), StatusCode::TEMPORARY_REDIRECT)
                })?;

                // add userid to endpoints that use this middleware
                req.extensions_mut().insert(UserId(user_id));

                // Skip CSRF check for safe methods like GET, HEAD, OPTIONS, TRACE
                if ["GET", "HEAD", "OPTIONS", "TRACE"].contains(&req.method().as_str()) {
                    return self.0.call(req).await;
                } else {
                }
                // go to next request if all is good
                self.0.call(req).await
            }

            None => Err(Error::from_string(
                "Cookie does not have a session_id key",
                StatusCode::BAD_REQUEST,
            )),
        }
    }
}
