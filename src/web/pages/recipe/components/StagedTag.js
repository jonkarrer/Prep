import { LitElement, html, css } from "/deps/lit.js";

export class StagedTag extends LitElement {
  constructor(name) {
    super();
    this.name = name;
    this.isChecked = false;
  }

  static properties = {
    name: { type: String },
    isChecked: { type: Boolean },
  };

  checkRecipeTagCheckbox(e) {
    let el = e.currentTarget;
    let checkbox = el.querySelector("input");
    let name = el.querySelector("p");

    checkbox.checked = !checkbox.checked;

    if (checkbox.checked) {
      name.classList.add("checked");
    } else {
      name.classList.remove("checked");
    }
  }

  firstUpdated() {
    let checkbox = this.querySelector("input[name=tag]");
    if (this.isChecked) checkbox.checked = true;
  }

  // render as light dom
  createRenderRoot() {
    return this;
  }

  render() {
    return html`
      <label class="StagedTag" @click="${this.checkRecipeTagCheckbox}">
        <p class="${this.isChecked ? "checked" : ""}">${this.name}</p>
        <input value=${this.name} name="tag" type="checkbox" />
      </label>
    `;
  }
}
