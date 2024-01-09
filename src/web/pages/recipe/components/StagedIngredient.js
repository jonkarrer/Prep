import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StagedIngredient extends LitElement {
  constructor(ingredient, amount, unit) {
    super();

    this.ingredient = ingredient;
    this.amount = amount;
    this.unit = unit;
  }

  // render as light dom
  createRenderRoot() {
    return this;
  }

  render() {
    return html`
      <div class="StagedIngredient">
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
}
