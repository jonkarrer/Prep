mod get;
mod post;
use poem::middleware::{AddData, AddDataEndpoint};
use poem::{get, post, EndpointExt, Route};

use self::get::{handle_get_recipe_by_id, health_check};
use self::post::{handle_create_recipe, handle_login, handle_register_user};

use super::MySqlDatabase;

pub fn router(database: MySqlDatabase) -> AddDataEndpoint<Route, &'static MySqlDatabase> {
    Route::new()
        .at("/health_check", get(health_check))
        .at("/recipe/:id", get(handle_get_recipe_by_id))
        .at("/register_user", post(handle_register_user))
        .at("/login_user", post(handle_login))
        .at("/create_recipe", post(handle_create_recipe))
        .with(AddData::new(&database))
}
