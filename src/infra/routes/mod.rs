mod auth;
mod get;
mod post;
use poem::{get, post, EndpointExt, Route};

use self::auth::{handle_login, handle_register};
use self::get::{handle_select_recipe_by_id, health_check};
use self::post::handle_create_recipe;

use super::middleware::AuthGuard;

pub fn router() -> Route {
    let recipe_routes = Route::new()
        .at("/select/:id", get(handle_select_recipe_by_id))
        .at("/create", post(handle_create_recipe))
        .with(AuthGuard);

    let user_routes = Route::new()
        .at("/register", post(handle_register))
        .at("/login", post(handle_login));

    let app = Route::new()
        .nest("/recipe", recipe_routes)
        .nest("/usr", user_routes)
        .at("/health_check", get(health_check));

    app
}
