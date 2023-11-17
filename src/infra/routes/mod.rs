mod auth;
mod health_check;
mod recipe;
mod usr;
use poem::endpoint::{StaticFileEndpoint, StaticFilesEndpoint};
use poem::{get, post, EndpointExt, Route};

use self::auth::{handle_login, handle_logout, handle_logout_ui, handle_register};
use self::health_check::handle_health_check;
use self::recipe::{handle_create_recipe, handle_get_recipe};
use self::usr::{
    handle_password_reset, handle_password_reset_ui, handle_update_email, handle_update_email_ui,
    handle_user_profile_details,
};

use super::middleware::AuthGuard;

pub fn router() -> Route {
    let recipe_routes = Route::new()
        .at("/select/:id", get(handle_get_recipe))
        .at("/create", post(handle_create_recipe))
        .with(AuthGuard);

    let auth_routes = Route::new()
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
        );

    let user_routes = Route::new()
        .at(
            "/dashboard",
            get(StaticFileEndpoint::new("src/web/templates/dashboard.html")),
        )
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
        .with(AuthGuard);

    let app = Route::new()
        .nest("/recipe", recipe_routes)
        .nest("/auth", auth_routes)
        .nest("/usr", user_routes)
        .nest("/", StaticFilesEndpoint::new("./src/web"))
        .at("/health_check", get(handle_health_check));

    app
}
