use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};

pub fn use_dash_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at(
            "/dash",
            get(StaticFileEndpoint::new("src/web/templates/dashboard.html")),
        )
        .with(AuthGuard)
}
