mod login;
mod logout;
mod register;

use crate::infra::middleware::AuthGuard;
use login::*;
use logout::*;
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};
use register::*;

pub fn use_auth_routes() -> Route {
    Route::new()
        .at(
            "/register",
            get(StaticFileEndpoint::new("src/web/templates/register.html")).post(handle_register),
        )
        .at(
            "/login",
            get(StaticFileEndpoint::new("src/web/templates/login.html")).post(handle_login),
        )
        .at(
            "/logout",
            get(handle_logout_ui).post(handle_logout).with(AuthGuard),
        )
}
