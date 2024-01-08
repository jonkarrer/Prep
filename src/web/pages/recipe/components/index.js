import { StagedIngredient } from "./StagedIngredient.js";
import { StagedDirection } from "./StagedDirection.js";
import { IngredientController } from "./IngredientController.js";
import { DirectionController } from "./DirectionController.js";
import { StaticIngredient } from "./StaticIngredient.js";
import { StaticDirection } from "./StaticDirection.js";
import { StaticTag } from "./StaticTag.js";

customElements.define("staged-ingredient", StagedIngredient);
customElements.define("staged-direction", StagedDirection);
customElements.define("ingredient-controller", IngredientController);
customElements.define("direction-controller", DirectionController);
customElements.define("static-ingredient", StaticIngredient);
customElements.define("static-direction", StaticDirection);
customElements.define("static-tag", StaticTag);
