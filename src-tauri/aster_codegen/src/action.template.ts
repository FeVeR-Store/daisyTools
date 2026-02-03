// @ts-nocheck
import { defineCard } from "../helper";

const $card$ = defineCard({
  branches: $branches$,
  parent: $parent$,
  name: $action_type$,
  args: $args$,
  litCardView: () => {
    return $stat$;
  },
  view: {
    title: "",
    form: $form$,
  },
  i18n: $i18n$,
});
