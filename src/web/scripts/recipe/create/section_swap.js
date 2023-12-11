import { updateRecipePreview } from "./recipe_preview.js";

let generalSection = document.getElementById("general_section");
let ingredientSection = document.getElementById("ingredient_section");
let ingredientController = document.getElementById("ingredient_controller");
let directionSection = document.getElementById("direction_section");
let directionController = document.getElementById("direction_controller");

function showGeneralSection() {
  generalSection.classList.remove("hidden");
  generalSection.classList.add("block");

  directionSection.classList.remove("block");
  directionController.classList.remove("block");
  directionSection.classList.add("hidden");
  directionController.classList.add("hidden");

  ingredientSection.classList.remove("block");
  ingredientSection.classList.add("hidden");
  ingredientController.classList.remove("block");
  ingredientController.classList.add("hidden");

  updateRecipePreview();
}

function showIngredientSection() {
  ingredientSection.classList.remove("hidden");
  ingredientController.classList.remove("hidden");
  ingredientSection.classList.add("block");
  ingredientController.classList.add("block");

  generalSection.classList.remove("block");
  generalSection.classList.add("hidden");
}

function showDirectionSection() {
  directionSection.classList.remove("hidden");
  directionController.classList.remove("hidden");
  directionSection.classList.add("block");
  directionController.classList.add("block");

  generalSection.classList.remove("block");
  generalSection.classList.add("hidden");

  ingredientSection.classList.remove("block");
  ingredientSection.classList.add("hidden");
  ingredientController.classList.remove("block");
  ingredientController.classList.add("hidden");
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
