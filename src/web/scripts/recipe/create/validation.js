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

export { runDirectionValidation, runDirectionValidation };
