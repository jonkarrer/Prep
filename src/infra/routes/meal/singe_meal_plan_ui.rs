use crate::{
    app::interface::{Database, MealPlanRepository, RecipeRepository},
    domain::entity::RecipeDetails,
};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Html, Path},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;
use tera::{Context, Tera};

#[handler]
pub async fn handle_single_meal_plan_ui(
    recipe_id: Path<String>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // Init template engine
    let tera = Tera::new("src/web/pages/meal/single/*.tera.html")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;

    // Fetch single meal plan
    let meal_plan = repo
        .select_meal_plan_by_id(&recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Get recipes in meal plan
    let mut recipes_in_plan = Vec::new();
    for recipe_id in meal_plan.recipe_ids {
        let recipe_details = repo.select_recipe_details_by_id(&recipe_id).await?;
        recipes_in_plan.push(recipe_details);
    }

    // Inject recipes into template
    let mut context = Context::new();
    context.insert("title", &meal_plan.meal_plan_name);
    context.insert::<Vec<RecipeDetails>, &str>("recipes", &recipes_in_plan);

    let rendered_html = tera
        .render("single_meal_plan.tera.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Html(rendered_html))
}
