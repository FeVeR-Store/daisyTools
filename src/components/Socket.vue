<template>
  <div>
    <Transition name="opacity">
      <article v-show="!dragState!.haveDragging" class="transition-all">
        <div
          class="card bg-base-200 text-primary-content w-full max-h-[calc(100vh-6rem)]"
        >
          <JsonTree
            v-if="litCard && litCard.plug"
            :model-value="{
              [`[${litCard.label}]`]:
                typeof litCard.plug === 'string'
                  ? litCard.plug
                  : {
                      ...litCard.plug,
                      '\0socket': true,
                    },
              '\0type': 'object',
            }"
            readonly
            draggable
            max-height="calc(100vh - 15rem)"
          />
        </div>
      </article>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import JsonTree from "./JsonTree.vue";
import { useDraggable } from "../utils/components/Draggable.vue";

const { litCard } = defineProps<{
  litCard: { label: string; plug: any };
}>();

const JsonNodeSymbol = Symbol.for("jsonNode");

// const currentCard = computed(() =>
//   litCard.find(({ id }) => id === active.value)
// );

const dragState = useDraggable(JsonNodeSymbol);
</script>
