<template>
  <main class="container">
    <!-- <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p> -->

    <OpenFolder v-if="currentPage === 'openFolder'" />
    <Setting v-if="currentPage === 'setting'" />

    <button @click="greet">greet</button>
  </main>
</template>

<script setup lang="ts">
  const hash = location.hash

  import { ref } from 'vue'
  import { invoke } from '@tauri-apps/api/core'

  import Setting from './pages/Setting.vue'
  import OpenFolder from './pages/OpenFolder.vue'

  const currentPage = hash ? hash.slice(2) : 'setting'

  const greetMsg = ref('')
  const name = ref('')

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke('greet', { name: name.value })
  }
</script>

<style scoped></style>

<style lang="less"></style>
