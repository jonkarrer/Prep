use crate::{
    app::interface::{Database, MealPlanRepository, RecipeRepository},
    domain::entity::{MealPlanArgs, RecipeDetails},
};
use brize_auth::entity::Session;
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Html, Json},
    Error, IntoResponse, Result,
};
use sqlx::MySqlPool;
use tera::{Context, Tera};

#[handler]
pub async fn handle_create_meal_plan_ui(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<impl IntoResponse> {
    // Init template engine
    let tera = Tera::new("src/web/pages/meal/create/*.tera.html")
        .map_err(|_| Error::from_status(StatusCode::NOT_FOUND))?;

    // Fetch all recipes
    let recipes = repo
        .select_all_recipe_details_for_user(&session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::NOT_FOUND))?;

    // Inject recipes into template
    let mut context = Context::new();
    context.insert::<Vec<RecipeDetails>, &str>("recipes", &recipes);

    let rendered_html = tera
        .render("create_meal_plan.tera.html", &context)
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // Serve template
    Ok(Html(rendered_html))
}

#[handler]
pub async fn handle_create_meal_plan(
    Json(meal_plan_args): Json<MealPlanArgs>,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<()> {
    repo.create_meal_plan(meal_plan_args, &session.user_id)
        .await?;

    Ok(())
}
