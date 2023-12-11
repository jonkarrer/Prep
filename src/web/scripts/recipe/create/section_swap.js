import { updateRecipePreview } from "./recipe_preview.js";

let generalSection = document.getElementById("general_section");
let ingredientSection = document.getElementById("ingredient_section");
let ingredientController = document.getElementById("ingredient_controller");
let directionSection = document.getElementById("direction_section");
let directionController = document.getElementById("direction_controller");

function showGeneralSection() {
  generalSection.classList.add("show");
  ingredientSection.classList.remove("show");
  ingredientController.classList.remove("show");
  directionSection.classList.remove("show");
  directionController.classList.remove("show");

  updateRecipePreview();
}

function showIngredientSection() {
  ingredientSection.classList.add("show");
  ingredientController.classList.add("show");
  generalSection.classList.remove("show");
  directionSection.classList.remove("show");
  directionController.classList.remove("show");
}

function showDirectionSection() {
  directionSection.classList.add("show");
  directionController.classList.add("show");
  ingredientSection.classList.remove("show");
  ingredientController.classList.remove("show");
  generalSection.classList.remove("show");
}

function showSection(section_id) {
  console.log("swap", section_id);
  switch (section_id) {
    case "general_section":
      showGeneralSection();
      break;

    case "ingredient_section":
      showIngredientSection();
      break;

    case "direction_section":
      showDirectionSection();
      break;

    default:
      break;
  }
}

export { showSection };
