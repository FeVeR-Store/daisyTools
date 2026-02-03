import { createApp } from "vue";
import App from "./App.vue";
import "./main.css";
import { router } from "./pages/pages";
import "./inject";
import { createI18n } from "vue-i18n";

const i18n = createI18n({
  legacy: false,
  locale: "zh-CN",
  fallbackLocale: {
    "zh-CN": ["zh_cn"],
  },
});

createApp(App).use(router).use(i18n).mount("#app");
