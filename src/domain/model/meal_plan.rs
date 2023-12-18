use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MealPlanModel {
    pub row_id: u64,
    pub meal_plan_id: String,
    pub user_id: String,
    pub meal_plan_name: String,
    pub created_at: String,
    pub updated_at: String,
}
