const TYPE_SYMBOL = "\0type";

type JsonNode<T extends "tuple" | "object" | "array"> =
  T extends "tuple"
    ? { "\0type": "tuple"; [key: `[${number}]`]: any }
    : T extends "object"
    ? { "\0type": "object"; [key: string]: any }
    : T extends "array"
    ? { "\0type": "array"; "[member]": any }
    : never;

export const defaultType = "string";
export const createTypeMap = () =>
  Object.freeze({
    string: "string",
    number: "number",
    boolean: "boolean",
    object: object({}),
    array: array(defaultType),
    tuple: tuple(1),
  } as const);

export const typeMapI18n = {
  "zh-CN": {
    string: "字符串",
    number: "数字",
    boolean: "布尔值",
    object: "对象",
    array: "数组",
    tuple: "元组",
  },
  en: {
    string: "string",
    number: "number",
    boolean: "boolean",
    object: "object",
    array: "array",
    tuple: "tuple",
  },
};

export function tuple(length: number): JsonNode<"tuple"> {
  const tuple = defineType({}, "tuple");
  new Array(length).fill(0).map((_, i) => {
    tuple[`[${i}]`] = defaultType;
  });
  return tuple;
}

export function object(obj: Record<string, any>): JsonNode<"object"> {
  return defineType(obj, "object");
}

export function array(type: any): JsonNode<"array"> {
  return defineType(
    {
      "[member]": type,
    },
    "array"
  );
}

function is(node: any, type: JsonNode<any>["\0type"]) {
  return typeof node === "object" && node[TYPE_SYMBOL] === type && type;
}
/** 判断是否为元组节点，如果是，则返回tuple */
export function isTuple(node: any) {
  return is(node, "tuple");
}
export function isObject(node: any) {
  return is(node, "object");
}
export function isArray(node: any) {
  return is(node, "array");
}

export function isTypeSymbol(key: string) {
  return key === TYPE_SYMBOL;
}
export function isJsonNode(node: any) {
  return TYPE_SYMBOL in node && node[TYPE_SYMBOL];
}

// export function isAccessSymbol(key: string) {
//   return key.startsWith("\0accessMember");
// }

function defineType<T extends "tuple" | "object" | "array">(
  node: any,
  type: T
): JsonNode<T> {
  // Object.defineProperty(node, "\0type", {
  //   value: type,
  //   enumerable: false,
  // });
  node[TYPE_SYMBOL] = type;
  return node;
}
