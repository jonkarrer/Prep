import { LitElement, html } from "./index.js";

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
    return html`<footer class="AppBar">
      <nav>
        <a
          hx-get="/dash"
          hx-target="#app"
          hx-swap="innerHTML"
          hx-push-url="true"
          >Home</a
        >
        <a
          hx-get="/recipe/all"
          hx-target="#app"
          hx-swap="innerHTML"
          hx-push-url="true"
          >Recipes</a
        >
        <a>Meals</a>
        <a>Pantry</a>
        <a
          hx-get="/usr/profile"
          hx-target="#app"
          hx-swap="innerHTML"
          hx-push-url="true"
          >Profile</a
        >
      </nav>
    </footer>`;
  }

  createRenderRoot() {
    return this;
  }
}
