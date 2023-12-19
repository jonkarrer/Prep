import { StagedIngredient } from "./StagedIngredient.js";
import { StagedDirection } from "./StagedDirection.js";
import { IngredientController } from "./IngredientController.js";
import { DirectionController } from "./DirectionController.js";

customElements.define("staged-ingredient", StagedIngredient);
customElements.define("staged-direction", StagedDirection);
customElements.define("ingredient-controller", IngredientController);
customElements.define("direction-controller", DirectionController);
