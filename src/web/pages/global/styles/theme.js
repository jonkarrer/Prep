import { css } from "/deps/lit.js";

export const theme = css`
  * {
    margin: 0;
    padding: 0;
  }

  a {
    text-underline-offset: 4px;
    text-decoration: underline;
    color: inherit;
  }

  h1,
  h2,
  h3,
  h4 {
    font-family: var(--pri-font);
  }

  p,
  a,
  input,
  button,
  ul,
  li,
  ol {
    font-family: var(--sec-font);
  }

  *,
  *::before,
  *::after {
    box-sizing: border-box;
  }
  body,
  html {
    overscroll-behavior-y: none;
  }

  input,
  button,
  select,
  textarea {
    font: inherit;
    outline: none;
    border: none;
  }
  ::placeholder {
    opacity: 0.5;
  }

  /* Fix mobile Safari increase font-size on landscape mode */
  html {
    -moz-text-size-adjust: none;
    -webkit-text-size-adjust: none;
    text-size-adjust: none;
  }

  /* Links */
  a {
    color: inherit;
  }
  *:any-link {
    color: inherit;
    text-decoration-line: none;
  }
  *:any-link:active {
    color: inherit;
    text-decoration-line: none;
  }

  /* Reapply the pointer cursor for anchor tags */
  a,
  button {
    cursor: pointer;
  }
  button:disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  /* Remove list styles (bullets/numbers) */
  ul,
  menu,
  summary {
    list-style: none;
  }

  /* For images to not be able to exceed their container */
  img {
    max-inline-size: 100%;
    max-block-size: 100%;
  }

  /* removes spacing between cells in tables */
  table {
    border-collapse: collapse;
  }

  /* Safari - solving issue when using user-select:none on the <body> text input doesn't working */
  input,
  textarea {
    -webkit-user-select: auto;
    user-select: auto;
  }

  /* Hide the spinner/arrow buttons in an input[type=number] */
  input[type="number"]::-webkit-inner-spin-button,
  input[type="number"]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  /* For Firefox */
  input[type="number"] {
    -moz-appearance: textfield;
    appearance: textfield;
  }

  /* revert the 'white-space' property for textarea elements on Safari */
  textarea {
    white-space: revert;
  }

  /* minimum style to allow to style meter element */
  meter {
    -webkit-appearance: revert;
    appearance: revert;
  }

  /* preformatted text - use only for this feature */
  :where(pre) {
    all: revert;
    box-sizing: border-box;
  }

  /* reset default text opacity of input placeholder */
  ::placeholder {
    color: unset;
  }

  /* fix the feature of 'hidden' attribute.
   display:revert; revert to element instead of attribute */
  :where([hidden]) {
    display: none;
  }

  /* Remove details summary webkit styles */
  ::-webkit-details-marker {
    display: none;
  }

  /* Hide scrollbar */
  ::-webkit-scrollbar {
    display: none;
    scrollbar-width: none;
  }
  ::-webkit-scrollbar-track {
    display: none;
  }
  ::-webkit-scrollbar-thumb {
    display: none;
  }
  ::-webkit-scrollbar-thumb:hover {
    display: none;
  }
`;
