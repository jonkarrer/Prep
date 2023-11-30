mod create_recipe;
mod get_all;
mod get_recipe;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use create_recipe::*;
use get_all::*;
use get_recipe::*;
use poem::{get, post, EndpointExt, Route};

pub fn use_recipe_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/select/:id", get(handle_get_recipe_ui))
        .at("/create", post(handle_create_recipe))
        .at("/all", get(handle_get_all_recipes_ui))
        .with(AuthGuard)
}
