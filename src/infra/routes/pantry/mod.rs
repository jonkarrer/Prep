mod all_pantry_items_ui;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use all_pantry_items_ui::*;
use poem::{get, EndpointExt, Route};

pub fn use_pantry_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_pantry_items_ui))
        .with(AuthGuard)
}
