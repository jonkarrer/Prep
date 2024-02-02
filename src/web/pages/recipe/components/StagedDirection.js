import { LitElement, html } from "/deps/lit.js";
// @pre
export class StagedDirection extends LitElement {
  constructor(detail) {
    super();

    this.detail = detail;
  }

  static properties = {
    detail: { type: String },
  };

  // render as light dom
  createRenderRoot() {
    return this;
  }

  firstUpdated() {
    const textarea = this.querySelector("#direction_input");
    const checkHeight = () => {
      if (textarea.scrollHeight > 0) {
        this.resize(textarea);
      } else {
        setTimeout(checkHeight, 50);
      }
    };

    checkHeight();
  }

  adjustTextareaSize(e) {
    this.resize(e.target);
  }

  resize(element) {
    element.style.height = "auto";
    element.style.height = element.scrollHeight + "px";
  }

  removeIngredient(e) {
    e.target.parentNode.remove();
  }

  // prettier-ignore
  render() {
    return html`
      <li class="StagedDirection">
        <button @click=${this.removeIngredient}>X</button>
        <textarea id="direction_input" name="direction" @input=${this.adjustTextareaSize}>${this.detail}</textarea> 
      </li>
    `;
  }
}
