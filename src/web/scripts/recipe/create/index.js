import { unitsFullNames } from "/assets/units.js";
import { ingredients } from "/assets/ingredients.js";
import { submitForm } from "./submit_form.js";
import { showSection } from "./section_swap.js";
import {
  createAutoCompleteHint,
  insertAutoCompleteIntoField,
} from "./autocomplete.js";

document.getElementById("recipe_form").addEventListener("submit", submitForm);

let unitAutoCompleteHint;
let ingredientAutoCompleteHint;
let unitInputController = document.getElementById("ingredient_controller_unit");
let ingredientInputController = document.getElementById(
  "ingredient_controller_ingredient"
);

unitInputController.addEventListener("input", (e) => {
  unitAutoCompleteHint = createAutoCompleteHint(
    e,
    "--unit-autocomplete",
    unitsFullNames
  );
});
ingredientInputController.addEventListener("input", (e) => {
  ingredientAutoCompleteHint = createAutoCompleteHint(
    e,
    "--ingredient-autocomplete",
    ingredients
  );
});

unitInputController.addEventListener("keydown", (e) =>
  insertAutoCompleteIntoField(e, unitAutoCompleteHint)
);
ingredientInputController.addEventListener("keydown", (e) =>
  insertAutoCompleteIntoField(e, ingredientAutoCompleteHint)
);

document.getElementById("general_nav").onclick = () =>
  showSection("general_section");
document.getElementById("ingredient_nav").onclick = () =>
  showSection("ingredient_section");
document.getElementById("direction_nav").onclick = () =>
  showSection("direction_section");
