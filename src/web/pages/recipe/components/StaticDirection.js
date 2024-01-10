import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StaticDirection extends LitElement {
  constructor(order, details) {
    super();

    this.order = order;
    this.details = details;
  }

  static properties = {
    order: { type: String },
    details: { type: String },
  };

  static styles = [
    theme,
    css`
      .direction {
        display: flex;
        align-items: baseline;
        gap: 10px;

        padding: 0.5rem 0;
      }
      .text {
        font-size: var(--rg);
        color: var(--contrast);
        line-height: 1.5;

        max-width: 100%;
        word-break: break-all;
      }
      .order {
        font-size: var(--rg);
        color: var(--accent);
        font-weight: bold;
      }
    `,
  ];

  render() {
    return html`
      <div class="direction">
        <div class="order">${this.order}</div>
        <p class="text">${this.details}</p>
      </div>
    `;
  }
}
