import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StagedIngredient extends LitElement {
  constructor(ingredient, amount, unit) {
    super();

    this.ingredient = ingredient;
    this.amount = amount;
    this.unit = unit;
  }

  static styles = [
    theme,
    css`
      div {
        border: solid green 1px;
        padding: 1rem;
        box-sizing: border-box;
      }
    `,
  ];

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
}
