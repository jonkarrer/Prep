import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StaticTag extends LitElement {
  constructor(tagName) {
    super();

    this.tagName = tagName;
  }

  static properties = {
    tagName: { type: String },
  };

  static styles = [
    theme,
    css`
      p {
        color: var(--sec-color);
        background-color: var(--accent);
        border: var(--solid-border);
        border-radius: var(--border-radius);

        padding: var(--button-padding);
      }
    `,
  ];

  render() {
    return html` <p>${this.tagName}</p>`;
  }
}
