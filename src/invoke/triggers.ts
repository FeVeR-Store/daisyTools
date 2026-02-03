import "cronstrue/locales/zh_CN";
import { registerDisplayName } from "./helper";
import { CardMeta } from "./type";

registerDisplayName("trigger")("time", {
  "zh-CN": "时间相关",
  en: "Time Related",
});

const modules = import.meta.glob("./triggers/*.{ts,tsx,js,jsx}", {
  eager: true,
  import: "default",
});

export const triggers = Object.values(modules) as CardMeta[];
