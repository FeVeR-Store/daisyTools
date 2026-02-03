import { FormItem, FormItemData, FormType, InputData } from "./Form.type";
import { createI18nWithUtils } from "../i18n/utils";
import { isPlug } from "./dataInput/PlugDisplay.utils";
import { createTypeMap } from "./JsonNode.utils";

type I18n = ReturnType<typeof createI18nWithUtils>;

function t(i18n: I18n, item: FormItem<any, FormItemData>, fallback: string[]) {
  const i18nInstance = i18n.global;
  let display = item.display;
  if (typeof item.display === "object") {
    // 当传入数组时，以参数列表形式传入并调用t
    if (Array.isArray(item.display)) {
      display = i18nInstance.t(...(item.display as [any])) as string;
    } else {
      // 当传入对象时，传入值，调用键 { t: "something" } => t("something")
      const key = Object.keys(item.display).find(
        (key) => key in i18nInstance
      ) as "t";
      let value = item.display[key];
      value = Array.isArray(value) ? value : [value];
      display = i18nInstance[key](...(value as [any])) as string;
    }
  } else {
    // 当传入字符串时，直接调用t，如果没有传入，则尝试fallback列表，直到找到可用翻译
    const tryList = [item.display, ...fallback];
    while (tryList.length > 0) {
      const currentKey = tryList.shift()!;
      if (!currentKey) continue;
      const result = i18nInstance.t(currentKey);
      if (result !== currentKey) {
        display = result;
        break;
      }
    }
  }
  // 如果最后没有找到翻译，那么使用最优先的fallback
  return (display ?? fallback[0]) as string;
}

export const processFormItem = (
  item: FormItem<any, FormItemData>,
  i18n: I18n
) => {
  processors.forEach((processor) => {
    if (processor.type === "formItem" && processor.active(item)) {
      processor.process(item as any, i18n);
    }
  });
};
export const processForm = (item: FormItem<any, FormItemData>, value: any) => {
  processors.forEach((processor) => {
    if (processor.type === "form" && processor.active(value, item)) {
      value = processor.process(value, item as any);
    }
  });
  return value;
};

type TypeMap = ReturnType<typeof createTypeMap>;

type PlugType = TypeMap[keyof TypeMap]; // extends infer R ? R extends string ? R : {  } : never;

export function typeCheck(plugType: PlugType, formType: FormType[number]) {
  console.log(plugType, formType);
  if (typeof plugType === "string") {
    switch (formType) {
      // case "Option":
      // case "Date":
      case "String":
      case "Code":
      case "TextArea":
      case "AutoComplete":
        return plugType === "string";
      case "Number":
        return plugType === "number";
      // case "File":
      case "Range":
      case "Switch":
        return plugType === "boolean";
    }
  }
  return false;
  // todo: 实现其他类型
}

const processors = [
  {
    type: "formItem",
    name: "form",
    active: () => true,
    process(item: FormItem<any, FormItemData>, i18n: I18n) {
      item.display = t(i18n, item, [item.name, `${item.name}.title`]);
    },
  },
  {
    type: "formItem",
    name: "input",
    active: (item: FormItem<any, FormItemData>) =>
      ["String", "Number"].includes(item.type),
    process(item: FormItem<any, InputData>, i18n: I18n) {
      // 处理placeholder
      if (!item.data?.placeholder) {
        // 如果placeholder为空，那么尝试[item.name].placeholder
        const fallback = `${item.name}.placeholder`;
        const placeholder = t(i18n, item, [fallback]);
        item.data ??= { placeholder: "" };
        // 如果placeholder与fallback相同，那么说明没有找到placeholder
        // 这种情况下，我们认为展示的placeholder并没有意义，因此置空
        if (placeholder === fallback) {
          item.data.placeholder = "";
        } else {
          // 如果placeholder与fallback不同，那么说明找到了placeholder
          item.data.placeholder = placeholder;
        }
      } else {
        item.data.placeholder = i18n.global.t(item.data.placeholder);
      }
    },
  },
  {
    type: "form",
    name: "input-number",
    active: (_, item) => item.type === "Number",
    process: Number as (value: any, item?: FormItem<any> | any) => any,
  },
  {
    type: "form",
    name: "plug",
    active: (value) => {
      return isPlug(value);
    },
    process: ([, ...value]: any) => {
      return {
        type: "Plug",
        value,
      };
    },
  },
] satisfies Processor[];

type Processor =
  | {
      name: string;
      type: "formItem";
      active: (item: FormItem<any>) => boolean;
      process: (item: FormItem<any> | any, i18n: I18n) => void;
    }
  | {
      name: string;
      type: "form";
      active: (value: any, item: FormItem<any>) => boolean;
      process: (value: any, item?: FormItem<any> | any) => any;
    };
