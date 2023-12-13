import { LitElement, html } from "./index.js";

export class AppBar extends LitElement {
  // Render the UI as a function of component state
  render() {
    return html`
      <div class="StagedIngredient">
        <input type="text" name="ingredient" placeholder="ingredient" />
        <input type="number" name="amount" placeholder="amount" />
        <input type="text" name="unit" placeholder="unit" />
      </div>
    `;
  }

  createRenderRoot() {
    return this;
  }
}
