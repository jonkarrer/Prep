import { LitElement, html } from "./index.js";

export class AppBar extends LitElement {
  constructor() {
    super();
  }

  render() {
    return html`
      <footer class="AppBar">
        <a href="/dash"> Home </a>
        <a href="/recipe/all"> Recipes </a>
        <div>Meals</div>
        <div>Pantry</div>
        <div href="/usr/profile">Profile</div>
      </footer>
    `;
  }

  createRenderRoot() {
    return this;
  }
}
