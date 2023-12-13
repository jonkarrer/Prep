import { LitElement, html, css } from "./index.js";

export class AppBar extends LitElement {
  constructor() {
    super();
  }

  static style = css`
    footer {
      position: fixed;
      bottom: 0;
      left: 0;

      display: flex;
      align-items: center;
      justify-content: space-between;

      box-sizing: border-box;
      padding: 0 1rem;
      width: 100%;
      height: 48px;

      background-color: var(--pri-color);
      color: var(--sec-color);
    }
  `;

  render() {
    return html`
      <footer>
        <a href="/dash"> Home </a>
        <a href="/recipe/all"> Recipes </a>
        <div>Meals</div>
        <div>Pantry</div>
        <div href="/usr/profile">Profile</div>
      </footer>
    `;
  }
}
