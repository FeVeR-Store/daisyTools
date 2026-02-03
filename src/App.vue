<script setup lang="ts">
import AppView from "./views/AppView.vue";
import TitleBar from "./components/TitleBar.vue";
import { mdiCogOutline } from "@mdi/js";
import Icon from "./components/Icon.vue";
</script>

<template>
  <AppView>
    <template #title-bar>
      <Suspense>
        <TitleBar>
          <template #left>
            <button
              @click="$router.push('/settings')"
              class="btn btn-square btn-ghost h-full"
            >
              <Icon :path="mdiCogOutline"></Icon>
            </button>
          </template>
        </TitleBar>
      </Suspense>
    </template>
    <main class="h-full">
      <RouterView></RouterView>
    </main>
    
  </AppView>
</template>

<style>
html {
  overflow: hidden;
}
</style>
