import { Data } from "../invoke/type";

/**
 * 将联合类型转为对应的交叉函数类型
 * @template U 联合类型
 */
type UnionToInterFunction<U> = (
  U extends any ? (k: () => U) => void : never
) extends (k: infer I) => void
  ? I
  : never;

/**
 * 获取联合类型中的最后一个类型
 * @template U 联合类型
 */
type GetUnionLast<U> = UnionToInterFunction<U> extends { (): infer A }
  ? A
  : never;

/**
 * 在元组类型中前置插入一个新的类型（元素）；
 * @template Tuple 元组类型
 * @template E 新的类型
 */
type Prepend<Tuple extends any[], E> = [E, ...Tuple];

/**
 * 联合类型转元组类型；
 * @template Union 联合类型
 * @template T 初始元组类型
 * @template Last 传入联合类型中的最后一个类型（元素），自动生成，内部使用
 */
export type UnionToTuple<
  Union,
  T extends any[] = [],
  Last = GetUnionLast<Union>
> = {
  0: T;
  1: UnionToTuple<Exclude<Union, Last>, Prepend<T, Last>>;
}[[Union] extends [never] ? 0 : 1];

/**
 * 联合类型转交叉类型
 * @template Union 联合类型
 */
export type UnionToIntersection<Union> = ((t: Union) => void) extends (
  t: infer I
) => void
  ? I
  : never;

export type Split<
  String extends string,
  Separator extends string
> = String extends `${infer A}${Separator}${infer B}` ? [A, B] : never;

export type NumberToArray<
  N extends number,
  C extends number[] = []
> = C["length"] extends N ? C : NumberToArray<N, [...C, 0]>;

export type Add<A extends number, B extends number> = [
  ...NumberToArray<A>,
  ...NumberToArray<B>
]["length"];

export type FindBy<
  key extends string,
  value extends any,
  From extends unknown[],
  current extends number = 0
> = key extends keyof From[current]
  ? From[current][key] extends value
    ? From[current]
    : Add<current, 1> extends number
    ? FindBy<key, value, From, Add<current, 1>>
    : never
  : never;

export type Optional<T, E extends keyof T> = Omit<T, E> & Partial<T>;

export type GetModelName<
  Provider extends (model: string, settings: any) => any
> = Provider extends (model: infer A, settings: any) => any ? A : never;

export type StartsWith<
  Str extends string,
  Pre extends string
> = Str extends `${Pre}${string}` ? true : false;

export function parseData<T>(data: Data): T {
  if (typeof data === "object" && "type" in data)
    switch (data.type) {
      case "String":
      case "Int":
      case "Float":
      case "Bool":
      case "Json":
      case "Null":
        return data.value as T;
      case "Plug":
        Reflect.defineMetadata("plug", true, data.value);
        return data.value as T;
    }
  return data;
}

export function toData(data: any): Data {
  let type: Data["type"];
  if (data === null) {
    return { type: "Null", value: null };
  }
  switch (typeof data) {
    case "string": {
      type = "String";
      break;
    }
    case "number": {
      if (data % 1 == 0) {
        type = "Int";
      } else {
        type = "Float";
      }
      break;
    }
    case "boolean":
      type = "Bool";
      break;
    case "undefined":
      type = "Null";
      break;
    case "object":
      type = "Json";
      break;
    default:
      type = "Null";
      console.error(`Unsupported data type ${typeof data}: ${data}`);
      break;
  }
  return { type, value: data };
}
