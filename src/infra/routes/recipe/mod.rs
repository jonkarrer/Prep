mod all_recipes_ui;
mod create_recipe;
mod single_recipe_ui;

use crate::{
    app::configs::StaticPath,
    infra::middleware::{AuthGuard, AuthGuardImpl},
};
use all_recipes_ui::*;
use create_recipe::*;
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};
use single_recipe_ui::*;

pub fn use_recipe_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_recipes_ui))
        .at("/select/:id", get(handle_single_recipe_ui))
        .at(
            "/create",
            get(StaticFileEndpoint::new(
                StaticPath::from("/pages/recipe/create_recipe.html").0,
            ))
            .post(handle_create_recipe),
        )
        .with(AuthGuard)
}
