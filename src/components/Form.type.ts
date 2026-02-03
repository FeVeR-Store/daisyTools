import { Slot, VNode } from "vue";

export type OptionData = {
  label: string;
  value: string;
}[];

export type FileData = {
  uploadApi: Function;
};

export type AutoCompleteData = {
  fetchSuggestions: (queryString: string, cb: (data: any) => void) => void;
};

export type RangeData = {
  min: number;
  max: number;
  step: number;
  display: (range: [number, number]) => string;
};

export type InputData = {
  placeholder: string;
};

export type FormItemData =
  | OptionData
  | FileData
  | AutoCompleteData
  | RangeData
  | InputData;
  
export type FormItem<
  T extends { [key: string]: any } = { [key: string]: any },
  D extends FormItemData = FormItemData
> = {
  optional?: boolean;
  name: keyof T & string;
  type: FormType[number];
  display?:
    | string
    | any[]
    | { [key in "t" | "rt" | "d" | "n" | "te" | "tm"]: any[] | string };
  defaultValue?: any;
  slot?: (res: Slot<any>, model: T, name: string) => VNode;
  tableSlot?: (res: Slot<any>, model: T, name: string) => VNode;
  data?: Partial<D>;
  plug?: string[] | null;
};
export type FormType = typeof formType;

export const formType = [
  "String",
  "Number",
  "Option",
  "Date",
  "TextArea",
  "File",
  "AutoComplete",
  "Range",
  "Code",
  "Switch",
] as const satisfies string[];

export type FormTypeValueMap = {
  String: string;
  Number: number;
  Option: string;
  Date: Date;
  TextArea: string;
  File: File;
  AutoComplete: string;
  Range: number;
  Code: string;
  Switch: boolean;
};
