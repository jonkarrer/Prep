use crate::domain::entity::RecipeArgs;

pub fn validate_recipe_args(recipe: &RecipeArgs) -> bool {
    let title = recipe.title.as_str();

    if title.len() == 0 || recipe.ingredients.len() == 0 || recipe.directions.len() == 0 {
        return false;
    }

    for dir in recipe.directions.iter() {
        if dir.details.len() == 0 {
            return false;
        }
    }

    for ing in recipe.ingredients.iter() {
        let ingredient_name = ing.name.as_str();

        if ingredient_name.len() == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::domain::entity::{DirectionArgs, IngredientArgs};

    use super::*;

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
