import { LitElement, html, css } from "/deps/lit.js";
import { StagedIngredient } from "./StagedIngredient.js";
import { createToast } from "/pages/global/components/Toast.js";
import { unitsFullNames } from "/assets/units.js";
import { ingredients } from "/assets/ingredients.js";

export class IngredientController extends LitElement {
  static styles = [
    css`
      :host() {
        display: none;
      }
      :host([show]) {
        display: block;
      }
      .Root {
        position: fixed;
        bottom: 48px;
        left: 0;
        right: 0;

        width: 80%;
        margin: auto;

        background-color: lightgrey;
        padding: 1rem;
      }
      .TopRow {
        display: flex;
        align-items: center;
        justify-content: space-between;
      }
      .CreateButton {
        background-color: green;
        color: white;
      }
      .IngredientController input {
        height: 1.8rem;
        width: 100%;

        background-color: rgba(0, 0, 0, 0.05);
        z-index: 1;
      }

      .IngredientWrapper {
        position: relative;
        margin-top: 1rem;
      }
      .AmountWrapper,
      .UnitWrapper {
        position: relative;
      }
      .AmountWrapper,
      .UnitWrapper {
        width: 40%;
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
      .IngredientWrapper::after {
        content: var(--ingredient-autocomplete);
      }
      .UnitWrapper::after {
        content: var(--unit-autocomplete);
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
        color: red;
        z-index: 1;
      }
    `,
  ];

  static properties = {
    unitAutoCompleteHint: "",
    ingredientAutoCompleteHint: "",
    fractionHint: "",
  };

  constructor() {
    super();

    this.amount = "";
    this.unit = "";
    this.ingredient = "";
  }

  createStagedIngredient() {
    console.log("stage ing");
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
    console.log("input", e.target.value);
    this.amount = e.target.value;
    this.fractionHint = this.showFractionHint(this.amount) ?? "";
  }

  handleAmountKeyPress(e) {
    console.log("keypress", e.target.value);
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
      <div class="Root">
        <div class="TopRow">
          <div class="AmountWrapper">
            <div class="AmountWrapper__fraction-hint">${this.fractionHint}</div>
            <input
              type="number"
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

          <span class="CreateButton" @click="${this.createStagedIngredient}">
            Plus
          </span>
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
      </div>
    `;
  }
}
