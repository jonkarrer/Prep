function roundToTenThousand(num) {
  return Math.round(num * 10000) / 10000;
}

function showFractionHint(decimal) {
  let wholeNumber = Math.floor(decimal);
  let decimalPart = roundToTenThousand(decimal - wholeNumber);

  const fractionMappings = new Map([
    [0.25, "1/4"],
    [0.75, "3/4"],
    [0.16, "1/6"],
    [0.125, "1/8"],
    [0.0625, "1/16"],
    [0.3, "1/3"],
    [0.33, "1/3"],
    [0.6, "2/3"],
    [0.66, "2/3"],
    [0.5, "1/2"],
    [0.2, "1/5"],
    [0.4, "2/5"],
    [0.6, "3/5"],
    [0.8, "4/5"],
  ]);

  return fractionMappings.get(parseFloat(decimalPart));
}

function handleAmountInput(e) {
  let fractionHint = document.getElementById("fraction_hint");

  let value = e.target.value;
  fractionHint.innerText = showFractionHint(value) ?? "";
}

function handleAmountKeyPress(e) {
  return ["e", "E", "+", "-", "/"].includes(e.key) && e.preventDefault();
}

export function useFractionHint() {
  let amountInput = document.getElementById("ingredient_controller_amount");

  amountInput.addEventListener("input", handleAmountInput);
  amountInput.addEventListener("keydown", handleAmountKeyPress);
}
