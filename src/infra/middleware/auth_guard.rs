use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};

use crate::infra::authentication::session_client;

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
const CSRF_TOKEN_KEY: &str = "X-CSRF-TOKEN";

// impl Endpoint trait for custom endpoint
#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthGuardImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        match req.header("Cookie") {
            Some(cookies) => {
                // find session_id in cookie str
                let session_str = cookies
                    .split(";")
                    .find(|x| x.contains(SESSION_COOKIE_KEY))
                    .ok_or(Error::from_string(
                        "Session id not found in cookie",
                        StatusCode::BAD_REQUEST,
                    ))?;

                // parse out the token
                let session_token = &session_str["session_id=".len()..];

                // validate session
                let session_details = session_client()
                    .await
                    .validate_session(&session_token)
                    .await
                    .map_err(|e| Error::from_string(format!("{}", e), StatusCode::UNAUTHORIZED))?;

                if ["GET", "OPTIONS", "HEAD"].contains(&req.method().as_str()) {
                    // pass session details to handler
                    req.extensions_mut().insert(session_details);
                    // call next route
                    return self.0.call(req).await;
                } else {
                    let con_type = req.content_type().ok_or(0).map_err(|e| {
                        Error::from_string(format!("{}", e), StatusCode::BAD_REQUEST)
                    })?;

                    if con_type == "application/json" {
                        let csrf_token = req.header(CSRF_TOKEN_KEY).ok_or(0).map_err(|e| {
                            Error::from_string(format!("{}", e), StatusCode::UNAUTHORIZED)
                        })?;

                        if !session_details.match_csrf_token(csrf_token) {
                            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
                        }
                    }

                    // pass session details to handler
                    req.extensions_mut().insert(session_details);
                    // call next route
                    return self.0.call(req).await;
                }
            }

            None => Err(Error::from_string(
                "Cookie does not have a session_id key",
                StatusCode::BAD_REQUEST,
            )),
        }
    }
}
