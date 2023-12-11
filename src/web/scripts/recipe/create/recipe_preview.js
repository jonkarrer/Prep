function updateRecipePreview() {
  removeStalePreviewItems();
  const form = document.getElementById("recipe_form");
  const formData = new FormData(form);

  const amounts = formData.getAll("amount");
  const units = formData.getAll("unit");
  const ingredients = formData.getAll("ingredient");
  const directions = formData.getAll("direction");

  const ingredientAnchor = document.getElementById("preview_ingredient_anchor");
  for (let i = 0; i < ingredients.length; i++) {
    let ingredientEl = document.createElement("p");
    let ingString = `${amounts[i]} ${units[i]} ${ingredients[i]}`;
    ingredientEl.innerText = ingString;
    ingredientAnchor.insertAdjacentElement("beforebegin", ingredientEl);
  }

  const directionAnchor = document.getElementById("preview_direction_anchor");
  for (let i = 0; i < directions.length; i++) {
    let directionEl = document.createElement("li");
    directionEl.innerText = directions[i];
    directionAnchor.insertAdjacentElement("beforebegin", directionEl);
  }
}

function removeStalePreviewItems() {
  let ingredientContainer = document.getElementById("ingredients_preview");
  let ingredients = ingredientContainer.querySelectorAll("p");

  for (let ing of ingredients) {
    ing.remove();
  }

  let directionContainer = document.getElementById("directions_preview");
  let directions = directionContainer.querySelectorAll("li");

  for (let dir of directions) {
    dir.remove();
  }
}

export { updateRecipePreview };
