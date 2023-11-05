mod get;
use get::health_check;

mod post;
use post::create_recipe;

use poem::{get, post, Route};
pub fn router() -> Route {
    Route::new()
        .at("/health_check", get(health_check))
        .at("/create_recipe", post(create_recipe))
}
