<script setup lang="ts">
import {
  mdiCardsClubOutline,
  mdiCardsDiamondOutline,
  mdiCardsHeartOutline,
  mdiCardsSpadeOutline,
} from "@mdi/js";
import Card from "../components/Card.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
</script>

<template>
  <div class="grid grid-cols-2 grid-rows-2 h-full gap-5">
    <Card
      @click="$router.push('/trigger')"
      :title="t('trigger.title')"
      :icon="mdiCardsClubOutline"
      class="bg-primary"
    >
    </Card>
    <Card
      @click="$router.push('/action')"
      :title="t('action.title')"
      :icon="mdiCardsDiamondOutline"
      class="bg-primary"
    ></Card>
    <Card
      @click="$router.push('/task')"
      :title="t('task.title')"
      :icon="mdiCardsHeartOutline"
      class="bg-primary"
      >
    </Card>
    <Card
    @click="$router.push('/example')"
      :title="t('widget.title')"
      :icon="mdiCardsSpadeOutline"
      class="bg-primary"
    >
    </Card>
  </div>
</template>

<i18n lang="yaml">
zh-CN:
  trigger:
    title: 触发器
  action:
    title: 动作
  task:
    title: 任务
  widget:
    title: 小组件
en:
  trigger:
    title: Trigger
  action:
    title: Action
  task:
    title: Task
  widget:
    title: Widget
</i18n>
