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
      .CreateButton {
        position: fixed;
        bottom: 2rem;
        left: 0;
        right: 0;
        margin: auto;
        background-color: var(--accent);
        border: var(--accent) 1px solid;
        border-radius: 100%;
        color: var(--sec-color);
        height: 4rem;
        width: 4rem;
      }
      .Overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vmax;

        display: none;
        z-index: 50;
      }
      .Overlay.open {
        display: block;
      }

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

    let amountInput = this.shadowRoot.querySelector("input[name='amount'");
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

  openController() {
    let controllerEl = this.shadowRoot.getElementById("ingredient_controller");
    let backgroundClickEl = this.shadowRoot.getElementById(
      "background_click_capture"
    );
    let amountInputEl = this.shadowRoot.querySelector("input[name=amount]");

    amountInputEl.focus();
    backgroundClickEl.classList.add("open");
    controllerEl.classList.add("open");
  }

  closeController() {
    let controllerEl = this.shadowRoot.getElementById("ingredient_controller");
    let backgroundClickEl = this.shadowRoot.getElementById(
      "background_click_capture"
    );

    backgroundClickEl.classList.remove("open");
    controllerEl.classList.remove("open");
  }

  render() {
    return html`
      <div
        class="Overlay"
        id="background_click_capture"
        @click="${this.closeController}"
      ></div>
      <div class="CreateButton" @click="${this.openController}">
        <svg
          width="100%"
          height="100%"
          viewBox="0 0 75 75"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <g clip-path="url(#clip0_2_60)">
            <path
              d="M41.25 33.75V18.75H33.75V33.75H18.75V41.25H33.75V56.25H41.25V41.25H56.25V33.75H41.25ZM37.5 75C27.5544 75 18.0161 71.0491 10.9835 64.0165C3.95088 56.9839 0 47.4456 0 37.5C0 27.5544 3.95088 18.0161 10.9835 10.9835C18.0161 3.95088 27.5544 0 37.5 0C47.4456 0 56.9839 3.95088 64.0165 10.9835C71.0491 18.0161 75 27.5544 75 37.5C75 47.4456 71.0491 56.9839 64.0165 64.0165C56.9839 71.0491 47.4456 75 37.5 75Z"
              fill="#e9f0f2"
            />
          </g>
          <defs>
            <clipPath id="clip0_2_60">
              <rect width="75" height="75" fill="black" />
            </clipPath>
          </defs>
        </svg>
      </div>
      <div id="ingredient_controller" class="Root">
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
