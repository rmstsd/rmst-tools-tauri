<template>
  <main class="container">
    <div class="drag-bar" data-tauri-drag-region></div>

    <!-- <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p> -->

    <OpenFolder v-if="currentPage === 'openFolder'" />
    <Setting v-if="currentPage === 'setting'" />
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

<style>
  .drag-bar {
    width: 100px;
    height: 100px;
    background-color: pink;
    user-select: none;
  }

  :root {
    color: #333;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    margin: 0;
  }

  * {
    box-sizing: border-box;
  }

  body {
    margin: 0;
  }
</style>
