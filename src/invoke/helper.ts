import { invoke } from "@tauri-apps/api/core";
import { api, invokeMap } from ".";
import { FindBy, Optional, UnionToTuple } from "../utils/type";
import { FormItem, formType, FormTypeValueMap } from "../components/Form.type";
import { useI18n } from "vue-i18n";
import { isPlug } from "../components/dataInput/PlugDisplay.utils";
import { StatProps } from "../components/Stat.vue";
import { merge } from "../utils/object";
import { CardMeta, Data, litCardViewProps } from "./type";

type InvokeMap = {
  [name: string]: {
    args: unknown[];
    return: unknown;
  };
};

type Invoke<T extends InvokeMap> = {
  [name in keyof T]: (...args: T[name]["args"]) => Promise<T[name]["return"]>;
};

export interface StatPropsWithKey<K extends string = string>
  extends Optional<StatProps, "title" | "description" | "value"> {
  key: K;
}

export function createInvoke<T extends InvokeMap>(map: T): Invoke<T> {
  const invokeMap: any = {};
  Object.keys(map).forEach((invokeName) => {
    invokeMap[invokeName] = (...args: any[]) => {
      const invokeArgs: any = {};
      let { args: invokeArgName } = map[invokeName];
      (invokeArgName as any[]).forEach((argName, i) => {
        invokeArgs[argName] = args[i];
      });
      return invoke(smallCamelToUnderline(invokeName), invokeArgs);
    };
  });
  return invokeMap;
}

export function createProxy<T extends CardMeta[]>(
  method: keyof typeof invokeMap
) {
  return new Proxy(
    {},
    {
      get(_, key: string) {
        return (name: string, args: Data) =>
          (api[method] as any)(key, name, args);
      },
    }
  ) as {
    [Name in T[number]["name"]]: (name: string, args: Data) => void;
  };
}

export function createDefaultForm<T extends { [key: string]: any }>(form: T) {
  return Object.keys(form).reduce((acc, key) => {
    switch (typeof form[key]) {
      case "string":
        acc[key] = "";
        break;
      case "number":
        acc[key] = 0;
        break;
      case "boolean":
        acc[key] = false;
        break;
    }
    return acc;
  }, {} as { [key: string]: any }) as T;
}

export function createDefaultFormFromFormItems<
  T extends { [key: string]: string }
>(formItems: FormItem<T>[]) {
  let res: { [key: string]: string } = {};
  formItems.forEach(({ name, type, defaultValue }) => {
    res[name] = formTypeToValue(type, defaultValue);
  });
  console.log(res);
  return res as T;
}

function formTypeToValue(type: (typeof formType)[number], defaultValue?: any) {
  if (defaultValue) return defaultValue;
  switch (type) {
    case "String":
      return "";
    case "Number":
      return 0;
    case "Option":
      return "";
    case "Date":
      return new Date();
    case "TextArea":
      return "";
    // case "File":
    //     return
    case "AutoComplete":
      return "";
    case "Range":
      return [0, 1];
    case "Code":
      return "\n\n\n";
  }
}

export const displayNameMap: { [key: string]: { [lang: string]: string } } = {};

export function registerDisplayName(namespace: string) {
  function addDisplayName(key: string, value: { [lang: string]: string }) {
    displayNameMap[`${namespace}.${key}`] = value;
    return addDisplayName;
  }
  return addDisplayName;
}

export function createGetDataExpose<T extends { [key: string]: any }>(
  formFormatter: <F extends Data>(form: T) => Data & F
) {
  return (props: { formData: T }) => ({
    getData() {
      const data = Object.fromEntries(
        Object.entries(props.formData).map(([key, value]) => {
          if (isPlug(value)) {
            return [
              key,
              {
                type: "Plug",
                value: {
                  type: value.type,
                  key: value,
                },
              },
            ];
          }
          return [key, value];
        })
      );
      return formFormatter(data as T) as any;
    },
  });
}

// 小驼峰转下划线命名法
export function smallCamelToUnderline(smallCamel: string) {
  return smallCamel.replace(/([A-Z])/g, "_$1").toLowerCase();
}

export function display(name: string) {
  const { locale } = useI18n({ useScope: "global" });
  console.log(locale.value);
  return displayNameMap[name][locale.value];
}

export function createActionInvokeMap<T extends CardMeta[]>(actions: T) {
  const map: {
    [key: string]: { [namespace: string]: (...args: any[]) => any };
  } = {};
  actions.forEach(({ name, parent, args: argsName }) => {
    const namespace = ["daisy", ...parent.split(".")].join("/");
    const func = (...args: any[]) => {
      api.runAction(name, createDataFromArgs(args, argsName as {} as string[]));
    };
    const apiName = name.replace("_action", "");
    if (namespace in map) {
      map[namespace][apiName] = func;
    } else {
      map[namespace] = { [apiName]: func };
    }
  });
  return map;
}

function createDataFromArgs(args: any[], argsName: string[]): Data {
  if (args.length === 1) {
    switch (typeof args) {
      case "string":
        return { type: "String", value: args };
      case "number":
      case "bigint":
        if (args % 1 === 0) {
          return { type: "Int", value: args };
        } else {
          return { type: "Float", value: args };
        }
      case "boolean":
        return { type: "Bool", value: args };
      case "undefined":
        return { type: "Null", value: null };
      case "object":
        return { type: "Json", value: args };
    }
  } else if (args.length === 0) {
    return { type: "Null", value: null };
  } else {
    const value: any = {};
    args.forEach((arg, i) => {
      value[argsName[i]] = arg;
    });
    console.log(args, argsName);
    return { type: "Json", value };
  }
}

// 用于自动补全的类型定义

/* 数据类型映射 */
type AC_DataMap = {
  Int: number;
  Float: number;
  String: string;
  Bool: boolean;
  Json: object;
  Null: null;
  Plug: string;
};

/* args定义，用于定义卡片的数据存储结构，并在编写时类型提示 */
type AC_Args = { [key: string]: Omit<Data, "Plug">["type"] | AC_Args };

/* 将args定义转化为ts类型 */
type AC_ArgsToData<Args extends AC_Args> = {
  [key in keyof Args]: Args[key] extends Omit<Data, "Plug">["type"]
    ? Args[key] extends keyof AC_DataMap
      ? AC_DataMap[Args[key]]
      : never
    : Args[key] extends AC_Args
    ? AC_ArgsToData<Args[key]>
    : never;
};

/* 将表单声明转化为表单数据 */
type AC_FormItemToForm<F extends FormItem[]> = {
  [name in F[number]["name"]]: FindBy<"name", name, F> extends infer R
    ? R extends FormItem
      ? FormTypeValueMap[R["type"]]
      : never
    : never;
};

type RequiredField = {
  [key in "title" | "description"]: string;
};

type AC_I18nAccpect = AC_Field | string | { [key: string]: AC_I18nAccpect };

type AC_Field = Partial<{
  [key in "title" | "description" | "placeholder" | "tooltip"]: AC_I18nAccpect;
}>;

type AC_I18n<F extends FormItem[] = FormItem[]> = Partial<{
  [key in keyof AC_FormItemToForm<F>]: AC_I18nAccpect;
}> &
  RequiredField & { [key: string]: AC_I18nAccpect };

type AC_ArgsToKey<F extends AC_Args, Prefix extends string = ""> = UnionToTuple<
  keyof F
> extends infer T
  ? T extends string[]
    ? {
        [i in keyof T]: T[i] extends keyof F
          ? F[T[i]] extends AC_Args
            ? AC_ArgsToKey<
                F[T[i]],
                Prefix extends "" ? T[i] & string : `${Prefix}.${T[i] & string}`
              >[number]
            : Prefix extends ""
            ? T[i]
            : `${Prefix}.${T[i] & string}`
          : never;
      }
    : never
  : never;

type AC_GetAllI18nPath<
  F extends object,
  Prefix extends string = ""
> = UnionToTuple<keyof F> extends infer T
  ? T extends string[]
    ? {
        [i in keyof T]: T[i] extends keyof F
          ? F[T[i]] extends string
            ? Prefix extends ""
              ? T[i]
              : `${Prefix}.${T[i] & string}`
            : F[T[i]] extends object
            ? AC_GetAllI18nPath<
                F[T[i]],
                Prefix extends "" ? T[i] & string : `${Prefix}.${T[i] & string}`
              >[number]
            : never
          : never;
      }
    : never
  : never;

type AC_I18nFormItem<I18nKey extends string[]> = {
  name: string;
  display: I18nKey;
  data?: Partial<{
    placeholder: I18nKey;
  }>;
  defaultValue?: any;
  plug?: string[] | null;
}[];

type AC_LitCardViewProps<
  Args extends AC_Args = AC_Args,
  F extends FormItem[] = FormItem[],
  I18n extends AC_I18n<F> = AC_I18n<F>
> = (
  props: litCardViewProps<AC_ArgsToData<Args>, I18n>
) => (
  | StatProps
  | StatPropsWithKey<F[number]["name"] | AC_ArgsToKey<Args>[number]>
)[];

type AC_View<
  Args extends AC_Args = AC_Args,
  F extends FormItem[] = FormItem[],
  I18n extends AC_I18n<F> = AC_I18n<F>
> = {
  title: string;
  form: F | AC_I18nFormItem<AC_GetAllI18nPath<I18n>[number]>;
  expose?: any;
  formFormatter?: (props: AC_FormItemToForm<F>) => AC_ArgsToData<Args>;
};
// | Renderable<CardComponentProps>;

type AC_Branch = {
  branch: string;
  type: string;
  id: string;
  position: string;
  plug?: any;
};

type AC_I18nMap<
  F extends FormItem[] = FormItem[],
  I18n extends AC_I18n<F> = AC_I18n<F>
> = {
  en: I18n;
  "zh-CN": I18n;
};
export const defineCard = <
  const P extends `action.${string}` | `trigger.${string}`,
  const Args extends AC_Args = AC_Args,
  const F extends FormItem[] = FormItem[],
  const I18n extends AC_I18n<F> = AC_I18n<F>
>(card: {
  branches: AC_Branch[];
  parent: P;
  name: string;
  args: Args;
  litCardView: AC_LitCardViewProps<Args, F, I18n>;
  view: AC_View<Args, F, I18n>;
  i18n: AC_I18nMap<F, I18n>;
}) => {
  return new Proxy(card, {
    get(target, p, receiver) {
      if (p === "extend") {
        return function (
          this: typeof target,
          extensiable: Partial<{
            litCardView: AC_LitCardViewProps<Args, F, I18n>;
            view: Partial<AC_View<Args, F, I18n>>;
            i18n: Partial<AC_I18nMap<F, I18n>>;
          }> = {}
        ) {
          if (extensiable.litCardView) {
            const ext = extensiable.litCardView;
            const origin = this.litCardView;
            this.litCardView = (...args) => {
              const originView = origin(...args);
              const extView = ext(...args);
              extView.forEach((item) => {
                // 首先，如果返回的view中包含key，那么就把相同key的合并
                if ("key" in item && item.key) {
                  const match = originView.find(
                    (i) => "key" in i && i.key == item.key
                  );
                  if (match) {
                    Object.assign(match, item);
                  }
                }
                // 否则，就添加到列表中
                else {
                  originView.push(item);
                }
              });
              return originView;
            };
          }
          if (extensiable.view) {
            const ext = extensiable.view;
            const origin = this.view;
            // 首先是form 这里根据name来合并
            ext.form?.forEach((item: any) => {
              const match = origin.form.find((i) => i.name == item.name);
              if (match) {
                Object.assign(match, item);
              } else {
                origin.form.push(item);
              }
            });
            // 然后是title
            if (ext.title) origin.title = ext.title;
            if (ext.formFormatter) origin.formFormatter = ext.formFormatter;
          }
          if (extensiable.i18n) {
            this.i18n = merge(this.i18n, extensiable.i18n);
          }
          return receiver;
        };
      } else if (p === "override") {
        return function (
          this: typeof target,
          extensiable: Partial<{
            parent: `action.${string}` | string;
            name: string;
            args: Args;
            litCardView: AC_LitCardViewProps<Args, F, I18n>;
            view: AC_View<Args, F, I18n>;
            i18n: AC_I18nMap<F, I18n>;
          }> = {}
        ) {
          // 遍历并覆盖
          Object.entries(extensiable).forEach(([key, value]) => {
            if (key in this) {
              this[key as keyof typeof this] = value as any;
            }
          });
          return receiver;
        };
      } else {
        return Reflect.get(target, p, receiver);
      }
    },
  }) as typeof card & {
    extend: (
      extensiable?: Partial<{
        litCardView: AC_LitCardViewProps<Args, F, I18n>;
        view: Partial<AC_View<Args, F, I18n>>;
        i18n: Partial<AC_I18nMap<F, I18n>>;
      }>
    ) => typeof card;
    override: (
      extensiable?: Partial<{
        parent: `action.${string}` | string;
        name: string;
        args: Args;
        litCardView: AC_LitCardViewProps<Args, F, I18n>;
        view: AC_View<Args, F, I18n>;
        i18n: AC_I18nMap<F, I18n>;
      }>
    ) => typeof card;
  };
};
