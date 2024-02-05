use crate::{
    app::clients::session_client, app::util::cookie_extractor,
    domain::constants::SESSION_COOKIE_KEY,
};
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

                req.extensions_mut().insert(session);
                return self.0.call(req).await;
            }

            None => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
        }
    }
}
