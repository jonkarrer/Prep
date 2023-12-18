mod all_meal_plans_ui;
mod singe_meal_plan_ui;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use all_meal_plans_ui::*;
use poem::{get, EndpointExt, Route};
use singe_meal_plan_ui::*;

pub fn use_meal_plan_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_meal_plans_ui))
        .at("/select/:id", get(handle_single_meal_plan_ui))
        .with(AuthGuard)
}
