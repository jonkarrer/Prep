import { LitElement, html, css } from "./index.js";

export class AppBar extends LitElement {
  static properties = {
    name: {},
  };

  constructor() {
    super();
    // Declare reactive properties
    this.name = "World";
  }

  // Render the UI as a function of component state
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
