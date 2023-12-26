mod all_pantry_items_ui;
mod create_pantry_item;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use all_pantry_items_ui::*;
use create_pantry_item::*;
use poem::{get, post, EndpointExt, Route};

pub fn use_pantry_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_pantry_items_ui))
        .at("/create", post(handle_create_pantry_item))
        .with(AuthGuard)
}
