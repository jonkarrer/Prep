mod handle_all_recipes_ui;
mod handle_create_recipe;
mod handle_single_recipe_ui;

use crate::{
    app::configs::StaticPath,
    infra::middleware::{AuthGuard, AuthGuardImpl},
};
use handle_all_recipes_ui::*;
use handle_create_recipe::*;
use handle_single_recipe_ui::*;
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};

pub fn use_recipe_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_recipes_ui))
        .at("/select/:id", get(handle_single_recipe_ui))
        .at(
            "/create",
            get(StaticFileEndpoint::new(
                StaticPath::from("/pages/recipe/create/create_recipe.html").0,
            ))
            .post(handle_create_recipe),
        )
        .with(AuthGuard)
}
