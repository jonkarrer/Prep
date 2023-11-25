use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};

pub fn use_dash_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at(
            "/",
            get(StaticFileEndpoint::new("src/web/pages/dashboard.html")),
        )
        .with(AuthGuard)
}
