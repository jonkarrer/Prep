mod get;
mod post;
use poem::{get, post, Route};

use self::get::{handle_get_recipe_by_id, health_check};
use self::post::{handle_create_recipe, handle_login, handle_register_user};
pub fn router() -> Route {
    Route::new()
        .at("/health_check", get(health_check))
        .at("/recipe/:id", get(handle_get_recipe_by_id))
        .at("/register_user", post(handle_register_user))
        .at("/login_user", post(handle_login))
        .at("/create_recipe", post(handle_create_recipe))
}
