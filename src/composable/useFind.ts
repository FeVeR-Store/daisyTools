import { MaybeRefOrGetter, toValue, watchEffect } from "vue";

export function useFind<T extends object, K extends (keyof T)[]>(
  array: MaybeRefOrGetter<T[]>,
  index: K
) {
  let indexMap = new Map<keyof T, Map<T[(typeof index)[number]], T>>();
  watchEffect(() => {
    indexMap.clear();
    toValue(array).forEach((ele) => {
      index.forEach((idx) => {
        if (indexMap.has(idx)) {
          indexMap.get(idx)!.set(ele[idx], ele);
        } else {
          indexMap.set(idx, new Map([[ele[idx], ele]]));
        }
      });
    });
  });
  return function (index: K[number], value: T[typeof index]) {
    if (indexMap.has(index)) {
      return indexMap.get(index)!.get(value);
    }
  };
}
