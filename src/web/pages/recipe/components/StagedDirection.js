import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StagedDirection extends LitElement {
  constructor(detail) {
    super();

    this.detail = detail;
  }
  static styles = [
    theme,
    css`
      textarea {
        border: solid green 1px;
      }
    `,
  ];

  // render as light dom
  createRenderRoot() {
    return this;
  }

  render() {
    return html`
      <li>
        <textarea name="direction">${this.detail}</textarea>
      </li>
    `;
  }
}
