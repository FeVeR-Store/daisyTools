import { onUnmounted } from "vue";
import { I18nOptions } from "vue-i18n";
import { createI18nWithUtils } from "../i18n/utils";

function createScopeI18n(option: I18nOptions) {
  function useScopeI18n() {
    const i18n = createI18nWithUtils(option);
    onUnmounted(() => {
      i18n.dispose();
    });
    return i18n.global;
  }
  return useScopeI18n;
}

export default createScopeI18n;
