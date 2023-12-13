export class AppBar extends HTMLElement {
  constructor() {
    super();
  }

  render() {
    this.innerHTML = `<footer class="AppBar">
    <a href="/dash">
      Home
    </a>
    <a href="/recipe/all">
      Recipes
    </a>
    <div>Meals</div>
    <div>Pantry</div>
    <div href="/usr/profile">
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
