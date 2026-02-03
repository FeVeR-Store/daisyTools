import { computed, MaybeRefOrGetter, toValue } from "vue";

export const createPlug = (plug: string[], oldValue: any) => {
  const p = plug as Plug;
  // 如果旧的值就是plug，那么就沿用旧值，避免类似堆栈式的remove
  if (oldValue["\0plug"]) {
    p["\0oldValue"] = oldValue["\0oldValue"];
    p["\0plug"] = true;
    return p;
  }
  p["\0plug"] = true;
  p["\0oldValue"] = oldValue;
  return p;
};

export interface Plug extends Array<string> {
  ["\0plug"]: true;
  ["\0oldValue"]: any;
}

export const getOldValue = (plug: Plug) => plug?.["\0oldValue"];

export const isPlug = (plug: any) => {
  return (
    plug &&
    (plug["\0plug"] ||
      (typeof plug === "object" && Reflect.getMetadata("plug", plug)))
  );
};

export function usePlug(modelValue: MaybeRefOrGetter<any | Plug>) {
  const plug = computed(() => {
    if (!isPlug(toValue(modelValue))) return null;
    return toValue(modelValue);
  });
  return {
    plug,
  };
}
export function usePlugPath(
  plug: MaybeRefOrGetter<Plug | string[]>,
  maxLength: MaybeRefOrGetter<number> = 3
) {
  const plugPath = computed(() => {
    return createPlugPath(plug, maxLength);
  });
  return {
    plugPath,
  };
}

export const createPlugPath = (
  plug: MaybeRefOrGetter<Plug | string[]>,
  maxLength: MaybeRefOrGetter<number> = 3
) => {
  const p = toValue(plug);
  const max = toValue(maxLength);
  const resultBefore = [];
  const resultAfter = [];
  if (p.length > max) {
    for (let i = 0; i < max; i++) {
      if (i % 2 === 0) {
        resultBefore.push(p.at(i / 2));
      } else {
        resultAfter.push(p.at(-(i + 1) / 2));
      }
    }
    return [...resultBefore, "...", ...resultAfter];
  }
  return p;
};
