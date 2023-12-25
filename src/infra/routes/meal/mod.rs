mod all_meal_plans_ui;
mod create_meal_plan;
mod singe_meal_plan_ui;

use crate::infra::middleware::{AuthGuard, AuthGuardImpl};
use all_meal_plans_ui::*;
use create_meal_plan::*;
use poem::{get, EndpointExt, Route};
use singe_meal_plan_ui::*;

pub fn use_meal_plan_routes() -> AuthGuardImpl<Route> {
    Route::new()
        .at("/all", get(handle_all_meal_plans_ui))
        .at("/select/:id", get(handle_single_meal_plan_ui))
        .at(
            "/create",
            get(handle_create_meal_plan_ui).post(handle_create_meal_plan),
        )
        .with(AuthGuard)
}
