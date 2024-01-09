import { LitElement, html } from "/deps/lit.js";

export class StagedDirection extends LitElement {
  constructor(detail) {
    super();

    this.detail = detail;
  }

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
