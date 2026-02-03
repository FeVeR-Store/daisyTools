import { I18n } from "vue-i18n";
import { StatProps } from "../components/Stat.vue";
import { Renderable } from "../utils/render";
import { StatPropsWithKey } from "./helper";
import createScopeI18n from "../composable/useScopeI18n";

type ActionEntry =
  | { LitRef: { id: string; wid: string } }
  | { Inline: { uid: string; type: string; data: Data } };

export type TaskMap = { [branchId: string]: ActionEntry };

type CardId = string;
type CardName = string;
type CardLabel = string;

enum Position {
  Left = "left",
  Right = "right",
  Top = "top",
  Bottom = "bottom",
}

export const argType = [
  "String",
  "Int",
  "Float",
  "Text",
  "File",
  "Code",
  "Bool",
] as const satisfies string[];

export type ArgType = (typeof argType)[number];

export type CardMeta = {
  branches: {
    branch: string;
    type: string;
    id: string;
    position: Position;
    plug: any;
  }[];
  parent: string;
  name: CardName;
  args: { [key: string]: ArgType };
  litCardView: (props: litCardViewProps) => (StatProps | StatPropsWithKey)[];
  view: Renderable<CardComponentProps>;
  i18n: {
    [lang: string]: {
      title: string;
      args_description?: string;
      description: string;
      [key: string]: any;
    };
  };
};

export type LitCard<T = any> = {
  id: CardId;
  label: CardLabel;
  type: string;
  data: T;
  plug: Record<string, any>;
};

export type IntType = { type: "Int"; value: number };
export type FloatType = { type: "Float"; value: number };
export type StringType = { type: "String"; value: string };
export type BoolType = { type: "Bool"; value: boolean };
export type JsonType<T extends object = object> = { type: "Json"; value: T };
export type NullType = { type: "Null"; value: null };
export type PlugType = {
  type: "Plug";
  value: {
    type: DataMap[keyof DataMap]["type"];
    key: string[];
  };
};

export type DataMap = {
  Int: IntType;
  Float: FloatType;
  String: StringType;
  Bool: BoolType;
  Json: JsonType;
  Null: NullType;
  Plug: PlugType;
};
export type Data<T extends keyof DataMap | object = keyof DataMap> =
  T extends object ? JsonType<T> : T extends keyof DataMap ? DataMap[T] : never;

export interface CardComponentProps {
  useI18n: ReturnType<typeof createScopeI18n>;
  cardInfo: CardMeta;
}

export type litCardViewProps<
  Args = any,
  I extends { [lang: string]: any } = {}
> = Omit<CardComponentProps, "useI18n"> & {
  litCardInfo: LitCard<Args>;
} & I18n<I, {}, {}, string, true>["global"];
