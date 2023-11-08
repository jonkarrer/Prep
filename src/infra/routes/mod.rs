mod get;
mod post;
use poem::endpoint::StaticFilesEndpoint;
use poem::{get, post, EndpointExt, Route};

use self::get::{handle_select_recipe_by_id, health_check};
use self::post::{handle_create_recipe, handle_login, handle_register_user};

use super::middleware::{AuthGuard, BasicAuth};

pub fn router() -> Route {
    let recipe_routes = Route::new()
        .at("/select/:id", get(handle_select_recipe_by_id))
        .at("/create", post(handle_create_recipe))
        .with(AuthGuard);

    let user_routes = Route::new()
        .at("/register", get(handle_register_user))
        .at("/login", get(handle_login))
        .with(BasicAuth);

    let app = Route::new()
        .nest("/recipe", recipe_routes)
        .nest("/usr", user_routes)
        .nest(
            "/",
            StaticFilesEndpoint::new("./src/web")
                .show_files_listing()
                .index_file("index.html"),
        )
        .at("/health_check", get(health_check));

    app
}
