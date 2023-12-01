mod create_recipe;
mod get_all_recipes;
mod get_single_recipe;

use crate::{
    app::configs::StaticPath,
    infra::middleware::{AuthGuard, AuthGuardImpl},
};
use create_recipe::*;
use get_all_recipes::*;
use get_single_recipe::*;
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};

pub fn use_recipe_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/select/:id", get(handle_get_single_recipe_ui))
        .at(
            "/create",
            get(StaticFileEndpoint::new(
                StaticPath::from("/pages/recipe/create_recipe.html").0,
            ))
            .post(handle_create_recipe),
        )
        .at("/all", get(handle_get_all_recipes_ui))
        .with(AuthGuard)
}
