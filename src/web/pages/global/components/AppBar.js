import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class AppBar extends LitElement {
  constructor() {
    super();
  }

  static styles = [
    theme,
    css`
      footer {
        position: fixed;
        bottom: 0;
        left: 0;

        display: flex;
        align-items: center;
        justify-content: space-between;

        padding: 0 1rem;
        width: 100%;
        height: 48px;

        background-color: var(--pri-color);
        color: var(--sec-color);
      }
    `,
  ];

  render() {
    return html`
      <footer>
        <a href="/dash"> Home </a>
        <a href="/recipe/all"> Recipes </a>
        <a href="/pantry/all"> Pantry </a>
        <a href="/meal/all"> Meals </a>
        <a href="/usr/profile">Profile</a>
      </footer>
    `;
  }
}
