import { LitElement, html, css } from "/deps/lit.js";

export class StagedTag extends LitElement {
  constructor(name) {
    super();
    this.name = name;
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

  // render as light dom
  createRenderRoot() {
    return this;
  }

  render() {
    return html`
      <label class="StagedTag" @click="${this.checkRecipeTagCheckbox}">
        <p>${this.name}</p>
        <input
          value=${this.name}
          name="tag"
          type="checkbox"
          checked=${this.isChecked}
        />
      </label>
    `;
  }
}
