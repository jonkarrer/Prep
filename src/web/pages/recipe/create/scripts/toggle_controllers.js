function openIngredientController() {
  let controllerEl = document.querySelector("ingredient-controller");
  let backgroundClickEl = document.getElementById("ingredient_bg_overlay");

  let amountInputEl =
    controllerEl.shadowRoot.querySelector("input[name=amount]");

  amountInputEl.focus();
  backgroundClickEl.classList.add("open");
  controllerEl.isOpen = true;
}

function closeIngredientController() {
  let controllerEl = document.querySelector("ingredient-controller");
  let backgroundClickEl = document.getElementById("ingredient_bg_overlay");

  backgroundClickEl.classList.remove("open");
  controllerEl.isOpen = false;
}

function openDirectionController() {
  let controllerEl = document.querySelector("direction-controller");
  let backgroundClickEl = document.getElementById("direction_bg_overlay");

  let textInputEl = controllerEl.shadowRoot.querySelector("textarea");

  textInputEl.focus();
  backgroundClickEl.classList.add("open");
  controllerEl.isOpen = true;
}

function closeDirectionController() {
  let controllerEl = document.querySelector("direction-controller");
  let backgroundClickEl = document.getElementById("direction_bg_overlay");

  backgroundClickEl.classList.remove("open");
  controllerEl.isOpen = false;
}

function openTagController() {
  let controllerEl = document.querySelector("tag-controller");
  let backgroundClickEl = document.getElementById("tag_bg_overlay");

  let textInputEl = controllerEl.shadowRoot.querySelector("input");

  textInputEl.focus();
  backgroundClickEl.classList.add("open");
  controllerEl.isOpen = true;
}

function closeTagController() {
  let controllerEl = document.querySelector("tag-controller");
  let backgroundClickEl = document.getElementById("tag_bg_overlay");

  backgroundClickEl.classList.remove("open");
  controllerEl.isOpen = false;
}

let openIngredientControllerEl = document.getElementById(
  "open_ingredient_controller_button"
);
let openDirectionControllerEl = document.getElementById(
  "open_direction_controller_button"
);
let openTagControllerEl = document.getElementById("open_tag_controller_button");

let ingredientOverlayEl = document.getElementById("ingredient_bg_overlay");
let directionOverlayEl = document.getElementById("direction_bg_overlay");
let tagOverlayEl = document.getElementById("tag_bg_overlay");

function useToggleControllers() {
  openIngredientControllerEl.onclick = () => openIngredientController();
  openDirectionControllerEl.onclick = () => openDirectionController();
  openTagControllerEl.onclick = () => openTagController();

  ingredientOverlayEl.onclick = () => closeIngredientController();
  directionOverlayEl.onclick = () => closeDirectionController();
  tagOverlayEl.onclick = () => closeTagController();
}

export {
  closeDirectionController,
  closeIngredientController,
  closeTagController,
  useToggleControllers,
};
