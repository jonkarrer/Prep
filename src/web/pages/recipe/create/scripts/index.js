import { useAutocomplete } from "./autocomplete.js";
import { useSubmitForm } from "./submit_form.js";
import { useSectionSwap } from "./section_swap.js";
import { useFractionHint } from "./fraction_hint.js";
import { useCreateStagedDirection } from "../components/StagedDirection.js";
import { useCreateStagedIngredient } from "../components/StagedIngredient.js";

useAutocomplete();
useSubmitForm();
useSectionSwap();
useCreateStagedDirection();
useCreateStagedIngredient();
useFractionHint();
