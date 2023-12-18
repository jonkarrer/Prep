use crate::domain::entity::MealPlan;
use anyhow::Result;

#[async_trait::async_trait]
pub trait MealPlanRepository: Send + Sync {
    async fn select_all_meal_plans(&self, user_id: &str) -> Result<Vec<MealPlan>>;
}
