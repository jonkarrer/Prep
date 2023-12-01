mod auth;
mod dash;
mod recipe;
mod templates;
mod user;
use poem::{endpoint::StaticFilesEndpoint, Route};

use crate::app::configs::StaticPath;

pub fn router() -> Route {
    Route::new()
        .nest("/recipe", recipe::use_recipe_routes())
        .nest("/auth", auth::use_auth_routes())
        .nest("/usr", user::use_user_routes())
        .nest("/dash", dash::use_dash_routes())
        .nest("/templates", templates::use_template_routes())
        .nest(
            "/",
            StaticFilesEndpoint::new(StaticPath::root().0).index_file("index.html"),
        )
}
