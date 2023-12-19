use crate::domain::entity::MealPlan;
use anyhow::Result;

#[async_trait::async_trait]
pub trait MealPlanRepository: Send + Sync {
    async fn select_all_meal_plans(&self, user_id: &str) -> Result<Vec<MealPlan>>;
    async fn select_meal_plan_by_id(&self, meal_plan_id: &str) -> Result<MealPlan>;
    async fn select_recipe_ids_for_plan(&self, meal_plan_id: &str) -> Result<Vec<String>>;
}
