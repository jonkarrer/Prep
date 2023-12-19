let generalSection = document.getElementById("general_section");
let ingredientSection = document.getElementById("ingredient_section");
let directionSection = document.getElementById("direction_section");

function showElements(elements) {
  for (let el of elements) {
    el.classList.add("show");
  }
}
function hideElements(elements) {
  for (let el of elements) {
    el.classList.remove("show");
  }
}

function hideIngredientSection() {
  hideElements([ingredientSection]);
}

function hideGeneralSection() {
  hideElements([generalSection]);
}

function hideDirectionSection() {
  hideElements([directionSection]);
}

function showGeneralSection() {
  showElements([generalSection]);
}

function showIngredientSection() {
  showElements([ingredientSection]);
}

function showDirectionSection() {
  showElements([directionSection]);
}

function showSection(section_id) {
  console.log("section", section_id);
  switch (section_id) {
    case "general_section":
      showGeneralSection();
      hideIngredientSection();
      hideDirectionSection();
      updateRecipePreview();
      break;

    case "ingredient_section":
      showIngredientSection();
      hideGeneralSection();
      hideDirectionSection();
      break;

    case "direction_section":
      showDirectionSection();
      hideGeneralSection();
      hideIngredientSection();
      break;

    default:
      break;
  }
}

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

export function useSectionSwap() {
  document.getElementById("general_nav").onclick = () =>
    showSection("general_section");
  document.getElementById("ingredient_nav").onclick = () =>
    showSection("ingredient_section");
  document.getElementById("direction_nav").onclick = () =>
    showSection("direction_section");
}
