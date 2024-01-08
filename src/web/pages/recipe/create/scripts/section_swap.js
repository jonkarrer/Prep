import { StaticDirection } from "../../components/StaticDirection.js";
import { StaticIngredient } from "../../components/StaticIngredient.js";

let generalSection = document.getElementById("general_section");
let ingredientSection = document.getElementById("ingredient_section");
let directionSection = document.getElementById("direction_section");

let generalNav = document.getElementById("general_nav");
let ingredientNav = document.getElementById("ingredient_nav");
let directionNav = document.getElementById("direction_nav");

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
  makeNaveInactive(ingredientNav);
  hideElements([ingredientSection]);
}

function hideGeneralSection() {
  makeNaveInactive(generalNav);
  hideElements([generalSection]);
}

function hideDirectionSection() {
  makeNaveInactive(directionNav);
  hideElements([directionSection]);
}

function showGeneralSection() {
  makeNavActive(generalNav);
  showElements([generalSection]);
}

function showIngredientSection() {
  makeNavActive(ingredientNav);
  showElements([ingredientSection]);
}

function showDirectionSection() {
  makeNavActive(directionNav);
  showElements([directionSection]);
}

function showSection(section_id) {
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
    let ingredientEl = new StaticIngredient(
      amounts[i],
      units[i],
      ingredients[i]
    );
    ingredientAnchor.insertAdjacentElement("beforebegin", ingredientEl);
  }

  const directionAnchor = document.getElementById("preview_direction_anchor");
  for (let i = 0; i < directions.length; i++) {
    let directionEl = new StaticDirection(i + 1, directions[i]);
    directionAnchor.insertAdjacentElement("beforebegin", directionEl);
  }
}

function removeStalePreviewItems() {
  let ingredients = document.querySelectorAll("static-ingredient");
  for (let ing of ingredients) {
    ing.remove();
  }

  let directions = document.querySelectorAll("static-direction");
  for (let dir of directions) {
    dir.remove();
  }
}

function makeNavActive(el) {
  el.classList.add("active");
}

function makeNaveInactive(el) {
  el.classList.remove("active");
}

export function useSectionSwap() {
  document.getElementById("general_nav").onclick = () =>
    showSection("general_section");
  document.getElementById("ingredient_nav").onclick = () =>
    showSection("ingredient_section");
  document.getElementById("direction_nav").onclick = () =>
    showSection("direction_section");
}
