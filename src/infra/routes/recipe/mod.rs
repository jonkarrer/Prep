mod create_recipe;
mod get_recipe;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use create_recipe::*;
use get_recipe::*;
use poem::{get, post, EndpointExt, Route};

pub fn use_recipe_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/select/:id", get(handle_get_recipe))
        .at("/create", post(handle_create_recipe))
        .with(AuthGuard)
}
