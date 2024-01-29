mod handle_all_recipes_ui;
mod handle_create_recipe;
mod handle_create_recipe_ui;
mod handle_delete_recipe;
mod handle_single_recipe_ui;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use handle_all_recipes_ui::*;
use handle_create_recipe::*;
use handle_create_recipe_ui::*;
use handle_delete_recipe::*;
use handle_single_recipe_ui::*;
use poem::{get, EndpointExt, Route};

pub fn use_recipe_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_recipes_ui))
        .at("/select/:id", get(handle_single_recipe_ui))
        .at(
            "/create",
            get(handle_create_recipe_ui).post(handle_create_recipe),
        )
        .at("/delete/:id", get(handle_delete_recipe))
        .with(AuthGuard)
}
