import { LitElement, html, css } from "/deps/lit.js";
import { createToast } from "/pages/global/components/Toast.js";
import { theme } from "/pages/global/styles/theme.js";
import { StagedDirection } from "./StagedDirection.js";

export class DirectionController extends LitElement {
  static styles = [
    theme,
    css`
      .Root {
        position: fixed;
        bottom: 48px;
        left: 0;
        right: 0;

        width: 80%;
        margin: auto;

        background-color: lightgrey;
        padding: 1rem;
      }
    `,
  ];

  constructor() {
    super();
    this.detail = "";
  }

  runDirectionValidation() {
    let lenCheck = (i) => i.length === 0;
    if (lenCheck(this.detail)) {
      createToast("error", "Direction field is empty");
      return false;
    }
    return true;
  }

  createStagedDirection(e) {
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
        <span
          class="IngredientController__add-button"
          @click="${this.createStagedDirection}"
        >
          Plus
        </span>
        <textarea
          @input="${this.handleDirectionInput}"
          placeholder="Input direction"
        ></textarea>
      </div>
    `;
  }
}
