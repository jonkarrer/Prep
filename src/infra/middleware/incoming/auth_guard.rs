use crate::{app::util::cookie_extractor, infra::authentication::session_client};
use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};

// name of middleware
pub struct AuthGuard;

// wrapper for custom endpoint
pub struct AuthGuardImpl<E>(E);

// impl Middlware trait for middleware struct
impl<E: Endpoint> Middleware<E> for AuthGuard {
    type Output = AuthGuardImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthGuardImpl(ep)
    }
}

const SESSION_COOKIE_KEY: &str = "session_id";
const CSRF_TOKEN_HEADER: &str = "X-CSRF-TOKEN";

// custom middleware that is passed to the handler
#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthGuardImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        match req.header("Cookie") {
            Some(cookies) => {
                let session_token = cookie_extractor(cookies, SESSION_COOKIE_KEY)
                    .ok_or(Error::from_status(StatusCode::UNAUTHORIZED))?;

                let session = session_client()
                    .await
                    .validate_session(&session_token)
                    .await
                    .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

                if ["GET", "OPTIONS", "HEAD"].contains(&req.method().as_str()) {
                    req.extensions_mut().insert(session);
                    return self.0.call(req).await;
                } else {
                    let content_type = req
                        .content_type()
                        .ok_or(0)
                        .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

                    if content_type == "application/json" {
                        let csrf_token = req
                            .header(CSRF_TOKEN_HEADER)
                            .ok_or(0)
                            .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

                        if !session.match_csrf_token(csrf_token) {
                            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
                        }
                    }

                    // pass session details to handler
                    req.extensions_mut().insert(session);

                    // call next route
                    return self.0.call(req).await;
                }
            }

            None => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
        }
    }
}
