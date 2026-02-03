<template>
  <div class="card-body flex flex-col gap-4">
    <h2 class="font-black text-3xl">请填写任务名</h2>
    <div>
      <p class="font-bold">您可以在任务页面查看所有的任务</p>
      <p>
        提交后的任务将立刻加入任务队列, 当触发器被激活时, 对应的行动将被执行
      </p>
    </div>
    <div>
      <Line :items class="relative bottom-3"></Line>
    </div>
    <div class="w-full px-2">
      <label class="input w-full rounded-3xl px-7">
        任务名称
        <input
          v-model="name"
          type="text"
          class="w-full"
          placeholder="请填写任务名"
        />
      </label>
      <button
        @click="createTask"
        name="subscribe"
        class="join-item btn btn-success rounded-3xl w-full mt-4"
      >
        <Icon :path="mdiCheck"></Icon>
        提交
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, Ref, ref } from "vue";
import Line from "./Line.vue";
import { api } from "../invoke";
import { mdiCheck } from "@mdi/js";
import Icon from "./Icon.vue";
import { toast } from "../utils/components/ToastProvider.vue";
import { LitCard } from "../invoke/type";
const trigger = inject<Ref<LitCard>>("trigger");
const action = inject<Ref<LitCard>>("action");
const items = computed(() =>
  [trigger!.value, action!.value].map((card) => card.label)
);

const name = ref("");
async function createTask() {
  try {
    await api.createTask(trigger!.value.id, name.value, [action!.value.id]);
    toast.success("任务创建成功");
  } catch (e) {
    toast.error("任务创建失败: " + e);
  }
}
</script>
