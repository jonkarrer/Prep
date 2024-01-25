import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";
import { StagedIngredient } from "./StagedIngredient.js";
import { createToast } from "/pages/global/components/Toast.js";
import { unitsFullNames } from "/assets/units.js";
import { ingredients } from "/assets/ingredients.js";

export class IngredientController extends LitElement {
  static styles = [
    theme,
    css`
      .Root {
        position: fixed;
        bottom: -100%;
        left: 0;
        right: 0;

        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-auto-flow: row;
        gap: 1rem;

        width: 90%;
        margin: auto;
        padding: 1rem;

        background-color: var(--sec-color);
        box-shadow: 1px 1px 2px 2px rgba(0, 0, 0, 0.2);
        border-radius: var(--border-radius);

        z-index: 1000;
      }
      .Root.open {
        bottom: 1rem;
      }
      .AmountWrapper {
        grid-column: 1;
      }
      .UnitWrapper {
        grid-column: 2;
      }
      .IngredientWrapper {
        grid-column: 1 / span 2;
      }

      button {
        padding: 0.5rem 0;
        font-size: var(--lg);

        background-color: var(--accent);
        color: var(--sec-color);
        border-radius: var(--border-radius);
        box-shadow: 1px 1px 2px 2px rgba(0, 0, 0, 0.1);

        grid-column: 1 / span 2;
      }

      input {
        padding: 0.6rem 1rem;
        width: 100%;

        border-radius: var(--border-radius);
        box-shadow: inset 1px 1px 1px 1px rgba(0, 0, 0, 0.2);
      }

      .AmountWrapper,
      .UnitWrapper,
      .IngredientWrapper {
        position: relative;
      }

      .IngredientWrapper::after,
      .UnitWrapper::after {
        position: absolute;
        left: 0;
        bottom: 0;

        height: 100%;
        width: 100%;

        display: flex;
        align-items: center;

        font-family: var(--sec-font);
        line-height: 1.2;

        opacity: 0.5;
        z-index: 0;
      }

      .AmountWrapper__fraction-hint {
        position: absolute;
        top: 0;
        right: 0;

        width: 2rem;
        height: 100%;
        padding-right: 5px;

        display: flex;
        align-items: center;

        background-color: rgba(0, 0, 0, 0);
        opacity: 0.5;

        text-align: center;
        font-family: var(--sec-font);
      }

      .UnitWrapper__base-input,
      .IngredientWrapper__base-input {
        position: absolute;
        top: 0;
        left: 0;
        background-color: rgba(0, 0, 0, 0);
        z-index: 2;
      }
      .UnitWrapper__autocomplete-input,
      .IngredientWrapper__autocomplete-input {
        z-index: 1;
        color: rgba(0, 0, 0, 0.3);
      }
    `,
  ];

  static properties = {
    unitAutoCompleteHint: "",
    ingredientAutoCompleteHint: "",
    fractionHint: "",
    isOpen: { type: Boolean },
  };

  constructor() {
    super();

    this.amount = "";
    this.unit = "";
    this.ingredient = "";
  }

  clearAndFocusInputs() {
    let inputs = this.shadowRoot.querySelectorAll("input");
    console.log("inputs", inputs);
    inputs.forEach((item) => (item.value = ""));
    this.amount = "";
    this.unit = "";
    this.ingredient = "";

    let amountInput = this.shadowRoot.querySelector("input[name=amount");
    amountInput.focus();
  }

  createStagedIngredient() {
    if (!this.runIngredientValidation()) {
      return;
    }

    let stagedIngredientEl = new StagedIngredient(
      this.ingredient,
      this.amount,
      this.unit
    );

    const anchor = document.getElementById("staged_ingredient_anchor");
    anchor.insertAdjacentElement("beforebegin", stagedIngredientEl);

    this.clearAndFocusInputs();
  }

  runIngredientValidation() {
    let lenCheck = (i) => i.length === 0;
    if (
      lenCheck(this.amount) ||
      lenCheck(this.unit) ||
      lenCheck(this.ingredient)
    ) {
      createToast("error", "inputs are empty");
      return false;
    }

    return true;
  }

  handleAmountInput(e) {
    this.amount = e.target.value;
    this.fractionHint = this.showFractionHint(this.amount) ?? "";
  }

  handleAmountKeyPress(e) {
    return ["e", "E", "+", "-", "/"].includes(e.key) && e.preventDefault();
  }

  showFractionHint(decimal) {
    let wholeNumber = Math.floor(decimal);
    let decimalPart = Math.round((decimal - wholeNumber) * 10000) / 10000;

    const fractionMappings = new Map([
      [0.25, "1/4"],
      [0.75, "3/4"],
      [0.16, "1/6"],
      [0.125, "1/8"],
      [0.0625, "1/16"],
      [0.3, "1/3"],
      [0.33, "1/3"],
      [0.6, "2/3"],
      [0.66, "2/3"],
      [0.5, "1/2"],
      [0.2, "1/5"],
      [0.4, "2/5"],
      [0.6, "3/5"],
      [0.8, "4/5"],
    ]);

    return fractionMappings.get(parseFloat(decimalPart));
  }

  filterAutoCompleteOptions(options, target) {
    return options.filter((item) => item.startsWith(target));
  }

  isCompletionKey(key) {
    return key === "Tab" || key === "Enter";
  }

  handleUnitInput(e) {
    this.unit = e.target.value;
    if (this.unit.length == 0) {
      this.unitAutoCompleteHint = "";
      return;
    }

    this.unitAutoCompleteHint =
      this.filterAutoCompleteOptions(unitsFullNames, this.unit)[0] ?? "";
  }

  handleUnitKeydown(e) {
    if (this.isCompletionKey(e.key)) {
      e.preventDefault();

      if (this.unitAutoCompleteHint.length != 0) {
        this.unit = this.unitAutoCompleteHint;
        e.target.value = this.unitAutoCompleteHint;
      }
    }
  }

  handleIngredientInput(e) {
    this.ingredient = e.target.value;
    if (this.ingredient.length == 0) {
      this.ingredientAutoCompleteHint = "";
      return;
    }

    this.ingredientAutoCompleteHint =
      this.filterAutoCompleteOptions(ingredients, this.ingredient)[0] ?? "";
  }

  handleIngredientKeydown(e) {
    if (this.isCompletionKey(e.key)) {
      e.preventDefault();

      if (this.ingredientAutoCompleteHint.length != 0) {
        this.ingredient = this.ingredientAutoCompleteHint;
        e.target.value = this.ingredientAutoCompleteHint;
      }
    }
  }

  render() {
    return html`
      <div class="Root ${this.isOpen ? "open" : ""}">
        <div class="AmountWrapper">
          <div class="AmountWrapper__fraction-hint">${this.fractionHint}</div>
          <input
            type="number"
            name="amount"
            placeholder="amount"
            @keydown="${this.handleAmountKeyPress}"
            @input="${this.handleAmountInput}"
          />
        </div>

        <div class="UnitWrapper">
          <input
            class="UnitWrapper__base-input"
            type="text"
            name="unit"
            placeholder="unit"
            @input="${this.handleUnitInput}"
            @keydown="${this.handleUnitKeydown}"
          />
          <input
            type="text"
            class="UnitWrapper__autocomplete-input"
            .value=${this.unitAutoCompleteHint || ""}
          />
        </div>

        <div class="IngredientWrapper">
          <input
            class="IngredientWrapper__base-input"
            type="text"
            name="ingredient"
            placeholder="ingredient"
            @input="${this.handleIngredientInput}"
            @keydown="${this.handleIngredientKeydown}"
          />
          <input
            type="text"
            class="IngredientWrapper__autocomplete-input"
            .value=${this.ingredientAutoCompleteHint || ""}
          />
        </div>
        <button @click="${this.createStagedIngredient}">Commit</button>
      </div>
    `;
  }
}
