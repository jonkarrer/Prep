import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StagedTag extends LitElement {
  static properties = {
    tagName: { type: String },
  };

  checkRecipeTagCheckbox(e) {
    let el = e.currentTarget;
    let checkbox = el.querySelector("input");
    let tagName = el.querySelector("p");

    checkbox.checked = !checkbox.checked;

    if (checkbox.checked) {
      tagName.classList.add("checked");
    } else {
      tagName.classList.remove("checked");
    }
  }

  // render as light dom
  createRenderRoot() {
    return this;
  }

  render() {
    return html`
      <label class="StagedTag" @click="${this.checkRecipeTagCheckbox}">
        <p>${this.tagName}</p>
        <input value=${this.tagName} name="tag" type="checkbox" />
      </label>
    `;
  }
}
