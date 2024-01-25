import { LitElement, html, css } from "/deps/lit.js";
import { createToast } from "/pages/global/components/Toast.js";
import { theme } from "/pages/global/styles/theme.js";
import { StagedTag } from "./StagedTag.js";

export class TagController extends LitElement {
  static styles = [
    theme,
    css`
      .Root {
        position: fixed;
        bottom: -100%;
        left: 0;
        right: 0;

        display: flex;
        flex-direction: column;
        gap: 1rem;

        width: 90%;
        margin: auto;
        padding: 1rem;

        background-color: var(--sec-color);
        box-shadow: 1px 1px 2px 2px rgba(0, 0, 0, 0.2);
        border-radius: var(--border-radius);

        z-index: 1000;
      }
      .Root.open {
        bottom: 1rem;
      }
      input {
        padding: 0.6rem 1rem;
        width: 100%;

        border-radius: var(--border-radius);
        box-shadow: inset 1px 1px 1px 1px rgba(0, 0, 0, 0.2);
      }
      button {
        padding: 0.5rem 0;
        font-size: var(--lg);

        background-color: var(--accent);
        color: var(--sec-color);
        border-radius: var(--border-radius);
        box-shadow: 1px 1px 2px 2px rgba(0, 0, 0, 0.1);
      }
    `,
  ];

  static properties = {
    isOpen: { type: Boolean },
  };

  constructor() {
    super();
    this.name = "";
  }

  runValidation() {
    let lenCheck = (i) => i.length === 0;
    if (lenCheck(this.name)) {
      createToast("error", "Direction field is empty");
      return false;
    }
    return true;
  }

  createNewTag() {
    if (!this.runValidation()) {
      return;
    }

    let stagedTagEl = new StagedTag(this.name);
    const anchor = document.getElementById("staged_tag_anchor");
    anchor.insertAdjacentElement("beforebegin", stagedTagEl);
  }

  handleDirectionInput(e) {
    this.name = e.target.value;
  }

  render() {
    return html`
      <div class="Root ${this.isOpen ? "open" : ""}">
        <input
          maxlength="60"
          @input="${this.handleDirectionInput}"
          placeholder="Tag Name"
        />
        <button
          class="IngredientController__add-button"
          @click="${this.createNewTag}"
        >
          Commit
        </button>
      </div>
    `;
  }
}
