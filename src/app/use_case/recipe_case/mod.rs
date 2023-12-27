mod create_recipe;
pub use create_recipe::*;

mod validate_recipe_args;
pub use validate_recipe_args::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::clients::db_client;
    use crate::app::helper::{get_test_recipe_args, get_test_session};
    use crate::domain::entity::{DirectionArgs, IngredientArgs, RecipeArgs};

    #[tokio::test]
    async fn test_case_create_recipe() {
        let repo = db_client().await;
        let recipe_args = get_test_recipe_args();
        let session = get_test_session().await.unwrap();

        let recipe = create_recipe(&repo, recipe_args, &session.user_id)
            .await
            .unwrap();

        assert_eq!(recipe.recipe_title, "Oatmeal")
    }

    #[test]
    fn test_case_validate_recipe_args() {
        let mut correct_recipe_args = RecipeArgs {
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
        };

        // Correct args passes
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(result);

        // No title fails
        correct_recipe_args.title = "".to_string();
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);

        // No ingredients fails
        correct_recipe_args.title = "some_title".to_string();
        correct_recipe_args.ingredients = vec![];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);

        // Empty ingredients fails
        correct_recipe_args.ingredients = vec![
            IngredientArgs {
                name: "".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
            IngredientArgs {
                name: "".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
        ];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);
        correct_recipe_args.ingredients = vec![IngredientArgs {
            name: "oats".to_string(),
            amount: 2.0,
            unit: "cups".to_string(),
        }];

        // No directions fails
        correct_recipe_args.directions = vec![];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);

        // Empty directions fails
        correct_recipe_args.directions = vec![
            DirectionArgs {
                details: "".to_string(),
                step_order: 1,
            },
            DirectionArgs {
                details: "".to_string(),
                step_order: 1,
            },
        ];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);
    }
}
