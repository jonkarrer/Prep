import { LitElement, html, css } from "/deps/lit.js";
import { createToast } from "/pages/global/components/Toast.js";
import { theme } from "/pages/global/styles/theme.js";
import { StagedDirection } from "./StagedDirection.js";

export class DirectionController extends LitElement {
  constructor() {
    super();
    this.detail = "";
  }

  static styles = [
    theme,
    css`
      .Root {
        position: fixed;
        bottom: 1rem;
        left: 0;
        right: 0;

        display: flex;
        flex-direction: column;
        gap: 1rem;

        width: 90%;
        margin: auto;
        padding: 1.5rem 1.5rem;

        box-shadow: 2px 2px 3px 3px rgba(0, 0, 0, 0.2);
        border-radius: var(--border-radius);
      }

      textarea {
        padding: 0.5rem 1rem;
        height: 120px;
        resize: none;
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

  runDirectionValidation() {
    let lenCheck = (i) => i.length === 0;
    if (lenCheck(this.detail)) {
      createToast("error", "Direction field is empty");
      return false;
    }
    return true;
  }

  createStagedDirection() {
    if (!this.runDirectionValidation()) {
      return;
    }

    let stagedDirectionEl = new StagedDirection(this.detail);
    const anchor = document.getElementById("staged_direction_anchor");
    anchor.insertAdjacentElement("beforebegin", stagedDirectionEl);
  }

  handleDirectionInput(e) {
    this.detail = e.target.value;
  }

  render() {
    return html`
      <div class="Root">
        <textarea
          maxlength="255"
          @input="${this.handleDirectionInput}"
          placeholder="Input direction"
        ></textarea>
        <button
          class="IngredientController__add-button"
          @click="${this.createStagedDirection}"
        >
          Commit
        </button>
      </div>
    `;
  }
}
