import { createToast } from "/pages/global/components/Toast.js";
import { LitElement, html, css } from "/deps/lit.js";
import { theme } from "/pages/global/styles/theme.js";

export class StagedDirection extends LitElement {
  constructor(detail) {
    super();

    this.detail = detail;
  }
  static styles = [
    theme,
    css`
      textarea {
        border: solid green 1px;
      }
    `,
  ];

  render() {
    return html`<textarea>${this.detail}</textarea>`;
  }
}

function runDirectionValidation(direction) {
  console.log("len", direction.length);
  let lenCheck = (i) => i.length === 0;
  if (lenCheck(direction)) {
    createToast("error", "Direction field is empty");
    return false;
  }
  return true;
}

function createStagedDirection(e) {
  const direction = document.getElementById(
    "direction_controller_direction"
  ).value;

  if (!runDirectionValidation(direction)) {
    return;
  }

  let stagedDirectionEl = new StagedDirection(direction);

  const anchor = document.getElementById("staged_direction_anchor");
  anchor.insertAdjacentElement("beforebegin", stagedDirectionEl);
}

export function useCreateStagedDirection() {
  document.getElementById("create_direction_button").onclick = (e) =>
    createStagedDirection(e);
}
