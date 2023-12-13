import { unitsFullNames } from "/assets/units.js";
import { ingredients } from "/assets/ingredients.js";

function setAutoCompleteIntoContent(cssVariableName, autoCompleteValue) {
  document.documentElement.style.setProperty(
    cssVariableName,
    `"${autoCompleteValue}"`
  );
}

function filterAutoCompleteOptions(options, target) {
  return options.filter((item) => item.startsWith(target));
}

function createAutoCompleteHint(e, cssVar, options) {
  let inputValue = e.target.value.toLowerCase();
  e.target.value = inputValue;

  if (inputValue.length === 0) {
    setAutoCompleteIntoContent(cssVar, "");
    return "";
  }

  let hint = filterAutoCompleteOptions(options, inputValue)[0] ?? "";
  setAutoCompleteIntoContent(cssVar, hint);
  return hint;
}

function insertAutoCompleteIntoField(e, autoCompleteValue) {
  if (e.key === "Tab" || e.key === "Enter") {
    e.preventDefault();

    if (autoCompleteValue.length != 0) {
      e.target.value = autoCompleteValue;
    }
  }
}

export function useAutocomplete() {
  let unitAutoCompleteHint;
  let ingredientAutoCompleteHint;
  let unitInputController = document.getElementById(
    "ingredient_controller_unit"
  );
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
}
