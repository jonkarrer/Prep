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
        let cookies = req.cookie();
        match cookies.get(SESSION_COOKIE_KEY) {
            Some(cookie) => {
                // get full string then extract just the uuid
                let session_str = cookie.to_string();
                let session_token = &session_str["session_id=".len()..];

                // validate session
                let mut auth = auth().await;
                let user_id = auth.validate_session(&session_token).await.map_err(|e| {
                    Error::from_string(format!("{e}"), StatusCode::TEMPORARY_REDIRECT)
                })?;

                // add userid to endpoints that use this middleware
                req.extensions_mut().insert(UserId(user_id));

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
