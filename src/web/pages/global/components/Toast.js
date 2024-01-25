import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class Toast extends LitElement {
  constructor() {
    super();
    this.message = "An error has occurred";
  }

  static styles = [
    theme,
    css`
      :host {
        position: absolute;
        top: 2rem;
        left: 0;
        right: 0;
        visibility: hidden;

        width: 70vw;
        margin: auto !important;
        padding: 1rem !important;

        border-radius: 1.3rem;

        color: var(--sec-color);
        font-weight: 500;
        text-align: center;
        z-index: 100000;
      }
      :host([error]) {
        background-color: rgb(154, 41, 41);
      }
      :host([warning]) {
        background-color: rgb(154, 145, 41);
      }

      :host([slide]) {
        animation: slideInDown 0.5s ease;
        animation-fill-mode: forwards;
      }
      @keyframes slideInDown {
        from {
          transform: translate3d(0, -120%, 0);
          visibility: visible;
        }

        to {
          transform: translate3d(0, 0, 0);
          visibility: visible;
        }
      }
    `,
  ];

  setMessage(message) {
    this.message = message ? message : "An Error Has Occurred";
  }

  render() {
    return html`<div class="toast">${this.message}</div>`;
  }
}

export function createToast(warningLevel, message) {
  const toastEl = new Toast();
  console.log("war", warningLevel, message);

  switch (warningLevel) {
    case "info":
      toastEl.setAttribute("info", "");
      break;

    case "warning":
      toastEl.setAttribute("warning", "");
      break;

    case "error":
      toastEl.setAttribute("error", "");
      break;

    default:
      break;
  }

  toastEl.setAttribute("slide", "");
  toastEl.setMessage(message);

  setTimeout(() => {
    toastEl.remove();
  }, 8000);

  document.body.appendChild(toastEl);
}
