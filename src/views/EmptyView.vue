<template>
  <div class="hero h-full w-full">
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="mb-5 text-8xl font-bold opacity-10 lg:text-7xl xl:text-9xl">
          {{ decoration }}
        </h1>
        <p class="my-10">{{ title }}</p>
        <slot></slot>
      </div>
    </div>
    <Icon
      class="absolute right-4 bottom-4 opacity-5 group-hover:scale-85 transition-all"
      size="120"
      :path="icon"
    ></Icon>
  </div>
</template>

<script setup lang="ts">
import Icon from "../components/Icon.vue";

defineProps<{
  decoration: string;
  title: string; 
  icon: string;
}>();
</script>
