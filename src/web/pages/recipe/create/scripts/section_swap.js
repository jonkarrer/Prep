import { StaticDirection } from "../../components/StaticDirection.js";
import { StaticIngredient } from "../../components/StaticIngredient.js";

let generalSection = document.getElementById("general_section");
let ingredientSection = document.getElementById("ingredient_section");
let directionSection = document.getElementById("direction_section");
let tagSection = document.getElementById("tag_section");

let generalNav = document.getElementById("general_nav");
let ingredientNav = document.getElementById("ingredient_nav");
let directionNav = document.getElementById("direction_nav");
let tagNav = document.getElementById("tag_nav");

function activateSection(sectionEl, navEl) {
  sectionEl.classList.add("show");
  navEl.classList.add("active");
}

function deactivateSection(sectionEl, navEl) {
  sectionEl.classList.remove("show");
  navEl.classList.remove("active");
}

function showSection(section_id) {
  switch (section_id) {
    case "general_section":
      activateSection(generalSection, generalNav);
      deactivateSection(ingredientSection, ingredientNav);
      deactivateSection(directionSection, directionNav);
      deactivateSection(tagSection, tagNav);
      updateRecipePreview();
      break;

    case "ingredient_section":
      activateSection(ingredientSection, ingredientNav);
      deactivateSection(generalSection, generalNav);
      deactivateSection(directionSection, directionNav);
      deactivateSection(tagSection, tagNav);
      break;

    case "direction_section":
      activateSection(directionSection, directionNav);
      deactivateSection(ingredientSection, ingredientNav);
      deactivateSection(generalSection, generalNav);
      deactivateSection(tagSection, tagNav);
      break;

    case "tag_section":
      activateSection(tagSection, tagNav);
      deactivateSection(directionSection, directionNav);
      deactivateSection(ingredientSection, ingredientNav);
      deactivateSection(generalSection, generalNav);
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

export function useSectionSwap() {
  generalNav.onclick = () => showSection("general_section");
  ingredientNav.onclick = () => showSection("ingredient_section");
  directionNav.onclick = () => showSection("direction_section");
  tagNav.onclick = () => showSection("tag_section");
}
