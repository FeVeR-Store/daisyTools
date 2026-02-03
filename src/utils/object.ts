interface DeepCloneOption {
  /** 需要克隆的键，如果键不可枚举，则需要在这里指定 */
  include: string[];
  /** 需要排除的键 */
  exclude: string[];
  /** 自定义处理 */
  process: (obj: any) => any;
}

export function deepClone<T>(obj: T, option?: Partial<DeepCloneOption>): T {
  switch (true) {
    // 基本类型直接返回
    case typeof obj !== "object" || obj === null:
      return obj;
    case Array.isArray(obj):
      return obj.map((member) => deepClone(member, option)) as T;
    default:
      const result: Record<string, any> = {};
      Object.entries(obj).forEach(([key, value]) => {
        if (option?.exclude?.includes(key)) {
          return;
        }
        result[key] = deepClone(value, option);
      });
      option?.include?.forEach((key) => {
        Object.defineProperty(result, key, {
          value: deepClone((obj as any)[key], option),
          writable: true,
          enumerable: false,
          configurable: true,
        });
      });
      return result as T;
  }
}

/**
 * Mr.DoNothing 什么也不做
 * 它允许你调用任何方法，但是返回值都是undefined\
 * 它允许你设置任何属性，但是返回值都是true\
 * 它允许你删除任何属性，但是返回值都是true\
 * 它允许你获取任何属性，但是返回值都是undefined\
 * 它允许你遍历任何属性，但是返回值都是[]\
 * 它允许你检查任何属性，但是返回值都是false\
 * todo: 可配置的行为
 */
export function mrDoNothing() {
  return new Proxy(
    {},
    {
      get() {
        return () => undefined;
      },
      set() {
        return true;
      },
      has() {
        return false;
      },
      deleteProperty() {
        return true;
      },
      ownKeys() {
        return [];
      },
    }
  );
}

/**
 * get 参考自lodash
 * @link https://github.com/lodash/lodash/blob/4.17.15/lodash.js#L13126
 */
export function get(
  object: { [key: string | symbol]: any },
  path: string | string[]
) {
  path = Array.isArray(path) ? path : stringToPath(path);

  let index = 0,
    length = path.length;

  while (object != null && index < length) {
    object = object[toKey(path[index++])];
  }
  return index && index == length ? object : undefined;
}

/**
 * toKey 参考自lodash
 * @link https://github.com/lodash/lodash/blob/4.17.15/lodash.js#L6753
 */
function toKey(value: any) {
  if (typeof value == "string" || typeof value === "symbol") {
    return value;
  }
  let result = value + "";
  // 转化-0
  // ? -0 转字符串也为 0, 因此需要判断
  return result == "0" && 1 / value == -Infinity ? "-0" : result;
}

/**
 * stringToPath 参考自lodash
 * @link https://github.com/lodash/lodash/blob/4.17.15/lodash.js#L6735
 */
function stringToPath(string: string) {
  const reEscapeChar = /\\(\\)?/g;
  const rePropName =
    /[^.[\]]+|\[(?:(-?\d+(?:\.\d+)?)|(["'])((?:(?!\2)[^\\]|\\.)*?)\2)\]|(?=(?:\.|\[\])(?:\.|\[\]|$))/g;

  const result = [];
  if (string.charCodeAt(0) === 46 /* . */) {
    result.push("");
  }
  string.replace(rePropName, (match, number, quote, subString) => {
    result.push(
      quote ? subString.replace(reEscapeChar, "$1") : number || match
    );
    return "";
  });
  return result;
}

export function merge<T extends Record<string, any>>(
  target: T,
  ...sources: Partial<T>[]
): T {
  if (!target || typeof target !== "object") {
    return target;
  }

  for (const source of sources) {
    if (!source || typeof source !== "object") {
      continue;
    }

    for (const key in source) {
      if (Object.prototype.hasOwnProperty.call(source, key)) {
        const sourceValue = source[key];
        const targetValue = target[key];

        if (isObject(sourceValue) && isObject(targetValue)) {
          // 如果两个值都是对象，递归合并
          // @ts-ignore
          target[key] = merge(targetValue, sourceValue);
        } else if (sourceValue !== undefined) {
          // 否则直接赋值（包括 null 值）
          // @ts-ignore
          target[key] = sourceValue;
        }
      }
    }
  }

  return target;
}

/**
 * 检查值是否为普通对象
 * @param value 要检查的值
 * @returns 是否为普通对象
 */
function isObject(value: any): value is Record<string, any> {
  return (
    value !== null &&
    typeof value === "object" &&
    !Array.isArray(value) &&
    !(value instanceof Date) &&
    !(value instanceof RegExp)
  );
}
