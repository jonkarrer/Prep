mod auth_route;
mod dash;
mod meal;
mod pantry_route;
mod recipe_route;
mod user;
use poem::{endpoint::StaticFilesEndpoint, Route};

use crate::app::configs::StaticPath;

pub fn router() -> Route {
    Route::new()
        .nest("/recipe", recipe_route::use_recipe_routes())
        .nest("/meal", meal::use_meal_plan_routes())
        .nest("/pantry", pantry_route::use_pantry_routes())
        .nest("/auth", auth_route::use_auth_routes())
        .nest("/usr", user::use_user_routes())
        .nest("/dash", dash::use_dash_routes())
        .nest(
            "/",
            StaticFilesEndpoint::new(StaticPath::root().0).index_file("index.html"),
        )
}
