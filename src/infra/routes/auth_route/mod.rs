mod handle_login;
mod handle_logout;
mod handle_register;

use crate::{app::configs::StaticPath, infra::middleware::AuthGuard};
use handle_login::*;
use handle_logout::*;
use handle_register::*;
use poem::{endpoint::StaticFileEndpoint, get, post, EndpointExt, Route};

pub fn use_auth_routes() -> Route {
    Route::new()
        .at(
            "/",
            get(StaticFileEndpoint::new(
                StaticPath::from("/pages/auth/auth.html").0,
            ))
            .post(handle_register),
        )
        .at("/register", get(handle_register_ui).post(handle_register))
        .at("/login", post(handle_login))
        .at(
            "/logout",
            get(handle_logout_ui).post(handle_logout).with(AuthGuard),
        )
}
