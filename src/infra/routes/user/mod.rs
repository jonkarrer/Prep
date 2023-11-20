mod password_reset;
mod profile_details;
mod update_email;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use password_reset::*;
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};
use profile_details::*;
use update_email::*;

pub fn use_user_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at(
            "/profile",
            get(StaticFileEndpoint::new("src/web/templates/profile.html")),
        )
        .at("/profile/details", get(handle_user_profile_details))
        .at(
            "/profile/password_reset",
            get(handle_password_reset_ui).put(handle_password_reset),
        )
        .at(
            "/profile/update_email",
            get(handle_update_email_ui).put(handle_update_email),
        )
        .with(AuthGuard)
}
