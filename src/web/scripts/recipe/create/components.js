import { handleToast } from "/scripts/utils/handle_toast.js";

function runDirectionValidation(direction) {
  console.log("len", direction.length);
  let lenCheck = (i) => i.length === 0;
  if (lenCheck(direction)) {
    handleToast("error", "direction field is empty");
    return false;
  }
  return true;
}

function runIngredientValidation(amount, unit, ingredient) {
  let lenCheck = (i) => i.length === 0;
  if (lenCheck(amount) || lenCheck(unit) || lenCheck(ingredient)) {
    handleToast("error", "inputs are empty");
    return false;
  }

  return true;
}

function createStagedIngredient(e) {
  const amount = document.getElementById("ingredient_controller_amount").value;
  const unit = document.getElementById("ingredient_controller_unit").value;
  const ingredient = document.getElementById(
    "ingredient_controller_ingredient"
  ).value;

  if (!runIngredientValidation(amount, unit, ingredient)) {
    console.log("invalid inputs");
    return;
  }

  let wrapper = document.createElement("div");
  wrapper.appendChild(clonedContent);

  // Set the values of the inputs
  wrapper.querySelector('input[name="ingredient"]').value = ingredient;
  wrapper.querySelector('input[name="amount"]').value = amount;
  wrapper.querySelector('input[name="unit"]').value = unit;

  const anchor = document.getElementById("staged_ingredient_anchor");
  anchor.insertAdjacentElement("beforebegin", wrapper);

  console.log("data", amount, unit, ingredient);
}

function createStagedDirection(e) {
  const direction = document.getElementById(
    "direction_controller_direction"
  ).value;

  if (!runDirectionValidation(direction)) {
    console.log("invalid direction");
    return;
  }

  let template = document.getElementById("RecipeDirection").content;
  let clonedContent = template.cloneNode(true);
  let wrapper = document.createElement("li");
  wrapper.appendChild(clonedContent);

  // Set the values of the inputs
  wrapper.querySelector('textarea[name="direction"]').value = direction;

  const anchor = document.getElementById("staged_direction_anchor");
  anchor.insertAdjacentElement("beforebegin", wrapper);
}

export function useCreateStagedDirection() {
  document.getElementById("create_direction_button").onclick = (e) =>
    createStagedDirection(e);
}
export function useCreateStagedIngredient() {
  document.getElementById("create_ingredient_button").onclick = (e) =>
    createStagedIngredient(e);
}
