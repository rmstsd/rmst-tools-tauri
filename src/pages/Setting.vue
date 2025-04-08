<template>
  <div>
    setting
    <button @click="openWin">open win</button>
  </div>
</template>

<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core'

  const openWin = () => {
    invoke('openWin')
  }
</script>

<style scoped></style>
