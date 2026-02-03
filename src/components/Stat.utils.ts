import { I18n } from "vue-i18n";
import { CardMeta } from "../invoke/type";
import { StatProps } from "../components/Stat.vue";
import { get } from "../utils/object";

export function normalizeStatProps(
  stat: ReturnType<CardMeta["litCardView"]>[number],
  form: { [key: string]: any },
  i18n: I18n<any, {}, {}, string, true>["global"] | any,
  args: CardMeta["args"]
) {
  // 统一化带key的stat
  if ("key" in stat) {
    const { key } = stat;
    // 利用key 我们可以推断出title, desc, value
    const inference = Object.fromEntries(
      ["title", "description"].map((item) => [item, i18n.t(`${key}.${item}`)])
    ) as StatProps;
    inference.value = get(form, key);
    inference.type = args[key];
    return { ...inference, ...stat }; // 允许覆盖推断属性
  }
  return stat;
}
