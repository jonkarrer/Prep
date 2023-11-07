mod get;
mod post;
use poem::{get, post, Route};

use self::get::{handle_select_recipe_by_id, health_check};
use self::post::{handle_create_recipe, handle_login, handle_register_user};

pub fn router() -> Route {
    let recipe_routes = Route::new()
        .at("/select/:id", get(handle_select_recipe_by_id))
        .at("/create", post(handle_create_recipe));

    let user_routes = Route::new()
        .at("/register", post(handle_register_user))
        .at("/login", post(handle_login));

    let app = Route::new()
        .nest("/recipe", recipe_routes)
        .nest("/usr", user_routes)
        .at("/health_check", get(health_check));

    app
}
