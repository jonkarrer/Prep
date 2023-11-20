mod auth;
mod dash;
mod health_check;
mod recipe;
mod user;

use self::health_check::handle_health_check;
use poem::{endpoint::StaticFilesEndpoint, get, Route};

pub fn router() -> Route {
    let app = Route::new()
        .nest("/recipe", recipe::use_recipe_routes())
        .nest("/auth", auth::use_auth_routes())
        .nest("/usr", user::use_user_routes())
        .nest("/dash", dash::use_dash_routes())
        .nest("/", StaticFilesEndpoint::new("./src/web"))
        .at("/health_check", get(handle_health_check));

    app
}
