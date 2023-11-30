use crate::{
    app::configs::StaticPath,
    infra::middleware::{AuthGuard, AuthGuardImpl},
};
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};

pub fn use_dash_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at(
            "/",
            get(StaticFileEndpoint::new(
                StaticPath::from("/pages/dashboard.html").0,
            )),
        )
        .with(AuthGuard)
}
