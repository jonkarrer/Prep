import { LitElement, html, css } from "/deps/lit.js";

export class Toast extends LitElement {
  constructor() {
    super();
    this.message = "An error has occurred";
  }

  static styles = css`
    :host {
      position: absolute;
      top: 2rem;
      left: 0;
      right: 0;
      visibility: hidden;

      width: max-content;
      max-width: 70vw;
      margin: auto;
      padding: 1rem;

      border-radius: 1.3rem;

      color: var(--sec-color);
      font-size: 0.8em;
      text-align: center;
      z-index: 1000;
    }
    :host([error]) {
      background-color: rgb(154, 41, 41);
    }
    :host([warning]) {
      background-color: rgb(154, 145, 41);
    }
  `;

  render() {
    return html` <div id="toast">${this.message}</div>`;
  }

  setMessage(message) {
    this.message = message;
  }
}

export function createToast(warningLevel, message) {
  const toastEl = new Toast();

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

  toastEl.classList.add("slideInDown");
  toastEl.setMessage(message);

  setTimeout(() => {
    toastEl.remove();
  }, 8000);

  document.body.appendChild(toastEl);
}
