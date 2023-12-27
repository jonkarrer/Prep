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
