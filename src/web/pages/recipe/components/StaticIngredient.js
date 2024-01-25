import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StaticIngredient extends LitElement {
  constructor(amount, unit, ingredient) {
    super();

    this.ingredient = ingredient;
    this.amount = amount;
    this.unit = unit;
  }

  static properties = {
    amount: { type: String },
    unit: { type: String },
    ingredient: { type: String },
  };

  static styles = [
    theme,
    css`
      .text {
        text-align: left;
        color: var(--contrast);
        padding: 0.5rem 0;
      }
      strong {
        font-weight: 600;
        color: var(--accent);
      }
    `,
  ];

  render() {
    return html`
      <div class="text">
        <strong>${this.amount}</strong> <strong>${this.unit}</strong>,
        ${this.ingredient}
      </div>
    `;
  }
}
