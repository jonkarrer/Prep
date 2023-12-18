mod all_meal_plans_ui;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use all_meal_plans_ui::*;
use poem::{get, EndpointExt, Route};

pub fn use_meal_plan_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_meal_plans_ui))
        .with(AuthGuard)
}
