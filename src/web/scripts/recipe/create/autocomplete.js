function setAutoCompleteIntoContent(cssVariableName, autoCompleteValue) {
  document.documentElement.style.setProperty(
    cssVariableName,
    `"${autoCompleteValue}"`
  );
}

function filterAutoCompleteOptions(options, target) {
  return options.filter((item) => item.startsWith(target));
}

function createAutoCompleteHint(e, cssVar, options) {
  let inputValue = e.target.value.toLowerCase();
  e.target.value = inputValue;

  if (inputValue.length === 0) {
    setAutoCompleteIntoContent(cssVar, "");
    return "";
  }

  let hint = filterAutoCompleteOptions(options, inputValue)[0] ?? "";
  setAutoCompleteIntoContent(cssVar, hint);
  return hint;
}

function insertAutoCompleteIntoField(e, autoCompleteValue) {
  if (e.key === "Tab" || e.key === "Enter") {
    e.preventDefault();

    if (autoCompleteValue.length != 0) {
      e.target.value = autoCompleteValue;
    }
  }
}

export { createAutoCompleteHint, insertAutoCompleteIntoField };
