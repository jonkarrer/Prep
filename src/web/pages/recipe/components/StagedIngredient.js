import { LitElement, html, css } from "/deps/lit.js";

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

  removeIngredient(e) {
    e.target.parentNode.remove();
  }

  render() {
    return html`
      <div class="StagedIngredient">
        <button @click=${this.removeIngredient}>X</button>
        <input
          type="number"
          name="amount"
          placeholder="amount"
          value=${this.amount}
        />
        <input type="text" name="unit" placeholder="unit" value=${this.unit} />
        <input
          type="text"
          name="ingredient"
          placeholder="ingredient"
          value=${this.ingredient}
        />
      </div>
    `;
  }
}
