mod get;
use get::health_check;

mod post;
use post::new_recipe;

use poem::{get, post, Route};
pub fn router() -> Route {
    Route::new()
        .at("/health_check", get(health_check))
        .at("/new_recipe", post(new_recipe))
}
