use crate::{
    app::interface::{Database, MealPlanRepository},
    domain::entity::{MealPlan, MealPlanDetails},
};
use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPool, Row};

#[async_trait::async_trait]
impl MealPlanRepository for Database<MySqlPool> {
    async fn select_all_meal_plans(&self, user_id: &str) -> Result<Vec<MealPlan>> {
        let mut meal_plans = Vec::new();

        let meal_plan_details: Vec<MealPlanDetails> = sqlx::query_as(
            r#"
            SELECT meal_plan_id, meal_plan_name
            FROM meal_plans
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .context("Could Not Select Meal Plans For User")?;

        for plan in &meal_plan_details {
            let recipe_ids = self.select_recipe_ids_for_plan(&plan.meal_plan_id).await?;

            meal_plans.push(MealPlan {
                meal_plan_id: plan.meal_plan_id.to_string(),
                meal_plan_name: plan.meal_plan_name.to_string(),
                recipe_ids,
            })
        }

        Ok(meal_plans)
    }

    async fn select_meal_plan_by_id(&self, meal_plan_id: &str) -> Result<MealPlan> {
        let details: MealPlanDetails = sqlx::query_as(
            r#"
            SELECT meal_plan_id, meal_plan_name
            FROM meal_plans
            WHERE meal_plan_id = ?
            "#,
        )
        .bind(meal_plan_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed To Get Meal Plan By Id")?;

        let recipe_ids = self.select_recipe_ids_for_plan(meal_plan_id).await?;

        Ok(MealPlan {
            meal_plan_id: details.meal_plan_id,
            meal_plan_name: details.meal_plan_name,
            recipe_ids,
        })
    }

    async fn select_recipe_ids_for_plan(&self, meal_plan_id: &str) -> Result<Vec<String>> {
        let recipe_ids: Vec<String> = sqlx::query(
            r#"
            SELECT recipe_id
            FROM meal_plans_to_recipes
            WHERE meal_plan_id = ?
            "#,
        )
        .bind(meal_plan_id)
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect();

        Ok(recipe_ids)
    }
}
