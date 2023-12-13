import { updateRecipePreview } from "./recipe_preview.js";

let generalSection = document.getElementById("general_section");
let ingredientSection = document.getElementById("ingredient_section");
let ingredientController = document.getElementById("ingredient_controller");
let directionSection = document.getElementById("direction_section");
let directionController = document.getElementById("direction_controller");

function showElements(elements) {
  for (let el of elements) {
    el.classList.remove("hidden");
    el.classList.add("block");
  }
}
function hideElements(elements) {
  for (let el of elements) {
    el.classList.remove("block");
    el.classList.add("hidden");
  }
}

function hideIngredientSection() {
  hideElements([ingredientSection, ingredientController]);
}

function hideGeneralSection() {
  hideElements([generalSection]);
}

function hideDirectionSection() {
  hideElements([directionSection, directionController]);
}

function showGeneralSection() {
  showElements([generalSection]);
}

function showIngredientSection() {
  showElements([ingredientSection, ingredientController]);
}

function showDirectionSection() {
  showElements([directionSection, directionController]);
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

export { showSection };
