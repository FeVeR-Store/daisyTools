import { onUnmounted, Ref, ref } from "vue";

const scopeStore = new Map<string | symbol, [number, any]>();

export function useStore<T>(id: string | symbol, defaultValue?: T) {
  if (scopeStore.has(id)) {
    const state = scopeStore.get(id);
    state![0]++;
    return state![1] as Ref<T | undefined>;
  }
  const store = ref(defaultValue);
  scopeStore.set(id, [1, store]);
  onUnmounted(() => {
    if (--scopeStore.get(id)![0] === 0) {
      scopeStore.delete(id);
    }
  });
  return store as Ref<T | undefined>;
}
