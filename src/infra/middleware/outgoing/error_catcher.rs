use poem::{
    async_trait, http::StatusCode, Endpoint, IntoResponse, Middleware, Request, Response, Result,
};

pub struct ErrorCatcher;

impl<E: Endpoint> Middleware<E> for ErrorCatcher {
    type Output = ErrorCatcherImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ErrorCatcherImpl(ep)
    }
}

pub struct ErrorCatcherImpl<E>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for ErrorCatcherImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let res = self.0.call(req).await;

        match res {
            Ok(resp) => Ok(resp.into_response()),
            Err(err) => match err.status() {
                StatusCode::UNAUTHORIZED => Ok(redirect_on_unauthorized()),
                _ => return Err(err),
            },
        }
    }
}

fn redirect_on_unauthorized() -> Response {
    Response::builder()
        .header("Location", "/auth")
        .status(StatusCode::SEE_OTHER)
        .finish()
}
