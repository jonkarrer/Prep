mod get;
use get::{health_check, recipe_by_id};

mod post;
use post::create_recipe;

use poem::{get, post, Route};
pub fn router() -> Route {
    Route::new()
        .at("/health_check", get(health_check))
        .at("/recipe/:id", get(recipe_by_id))
        .at("/create_recipe", post(create_recipe))
}
