import { useAutocomplete } from "./autocomplete.js";
import { useSubmitForm } from "./submit_form.js";
import { useSectionSwap } from "./section_swap.js";
import {
  useCreateStagedDirection,
  useCreateStagedIngredient,
} from "./components.js";
import { useFractionHint } from "./fraction_hint.js";

useAutocomplete();
useSubmitForm();
useSectionSwap();
useCreateStagedDirection();
useCreateStagedIngredient();
useFractionHint();
