export class AppBar extends HTMLElement {
  constructor() {
    super();
  }

  render() {
    this.innerHTML = `<footer class="AppBar">
    <button
      hx-get="/dash"
      hx-trigger="click"
      hx-target="#app"
      hx-swap="innerHTML"
      hx-push-url="true"
    >
      Home
    </button>
    <button
      hx-get="/recipe/all"
      hx-trigger="click"
      hx-target="#app"
      hx-swap="innerHTML"
      hx-push-url="true"
    >
      Recipes
    </button>
    <div>Meals</div>
    <div>Pantry</div>
    <div
      hx-get="/usr/profile"
      hx-trigger="click"
      hx-target="#app"
      hx-swap="innerHTML"
      hx-push-url="true"
    >
      Profile
    </div>
  </footer>`;
  }

  connectedCallback() {
    if (!this.rendered) {
      this.render();
      this.rendered = true;
    }
  }
}
