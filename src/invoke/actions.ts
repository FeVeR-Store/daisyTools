import { registerDisplayName } from "./helper";
import { CardMeta } from "./type";

registerDisplayName("action")("program", {
  "zh-CN": "可编程",
  en: "Programmable",
})("web", {
  "zh-CN": "网络相关",
  en: "Web Related",
})("debug", {
  "zh-CN": "调试使用",
});

const modules = import.meta.glob("./actions/*.{ts,tsx,js,jsx}", {
  eager: true,
  import: "default",
});

export const actions = Object.values(modules) as CardMeta[];
