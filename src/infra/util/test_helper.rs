use crate::{
    application::interface::Database,
    domain::entity::{DirectionArgs, IngredientArgs, RecipeArgs},
    infra::db,
};
use poem::{
    middleware::{AddData, AddDataEndpoint},
    test::TestClient,
    EndpointExt, Route, RouteMethod,
};
use sqlx::{MySql, Pool};

pub fn init_test_client(route: &str, handler: RouteMethod) -> TestClient<Route> {
    let app = Route::new().at(route, handler);
    TestClient::new(app)
}

pub async fn init_test_client_with_db(
    route: &str,
    handler: RouteMethod,
) -> TestClient<AddDataEndpoint<Route, Database<Pool<MySql>>>> {
    let app = Route::new()
        .at(route, handler)
        .with(AddData::new(db().await));

    TestClient::new(app)
}

pub fn get_test_recipe_args() -> RecipeArgs {
    RecipeArgs {
        title: "Oatmeal".to_string(),
        servings: 2.0,
        favorite: true,
        tags: vec!["vegan".to_string()],
        ingredients: vec![
            IngredientArgs {
                name: "oats".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
            IngredientArgs {
                name: "milk".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
        ],
        directions: vec![
            DirectionArgs {
                details: "boil and stir".to_string(),
                step_order: 1,
            },
            DirectionArgs {
                details: "enjoy and stir".to_string(),
                step_order: 1,
            },
        ],
    }
}
