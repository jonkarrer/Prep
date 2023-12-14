import { createToast } from "/pages/global/components/Toast.js";
import { LitElement, html, css } from "/deps/lit.js";

export class StagedIngredient extends LitElement {
  constructor(ingredient, amount, unit) {
    super();

    this.ingredient = ingredient;
    this.amount = amount;
    this.unit = unit;
  }

  static styles = css`
    div {
      border: solid green 1px;
      padding: 1rem;
      box-sizing: border-box;
    }
  `;

  render() {
    return html`
      <div>
        <input
          type="text"
          name="ingredient"
          placeholder="ingredient"
          value=${this.ingredient}
        />
        <input
          type="number"
          name="amount"
          placeholder="amount"
          value=${this.amount}
        />
        <input type="text" name="unit" placeholder="unit" value=${this.unit} />
      </div>
    `;
  }

  setIngredient(ingredient) {
    this.ingredient = ingredient;
  }

  setAmount(amount) {
    this.amount = amount;
  }

  setUnit(unit) {
    this.unit = unit;
  }
}

function runIngredientValidation(amount, unit, ingredient) {
  let lenCheck = (i) => i.length === 0;
  if (lenCheck(amount) || lenCheck(unit) || lenCheck(ingredient)) {
    createToast("error", "inputs are empty");
    return false;
  }

  return true;
}

function createStagedIngredient(e) {
  console.log("stage ing");
  const amount = document.getElementById("ingredient_controller_amount").value;
  const unit = document.getElementById("ingredient_controller_unit").value;
  const ingredient = document.getElementById(
    "ingredient_controller_ingredient"
  ).value;

  if (!runIngredientValidation(amount, unit, ingredient)) {
    return;
  }

  let stagedIngredientEl = new StagedIngredient(ingredient, amount, unit);

  const anchor = document.getElementById("staged_ingredient_anchor");
  anchor.insertAdjacentElement("beforebegin", stagedIngredientEl);
}

export function useCreateStagedIngredient() {
  document.getElementById("create_ingredient_button").onclick = (e) =>
    createStagedIngredient(e);
}
