mod login;
mod logout;
mod register;

use crate::{app::configs::StaticPath, infra::middleware::AuthGuard};
use login::*;
use logout::*;
use poem::{endpoint::StaticFileEndpoint, get, post, EndpointExt, Route};
use register::*;

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
