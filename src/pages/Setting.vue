<template>
  <div>
    <div>setting</div>

    <button @click="openWin">open win</button>

    <button @click="saveHandler">保存</button>
    <button @click="importSetting">导入</button>
    <button @click="exportSetting">导出</button>

    <input type="text" v-model="state.formValues.cmdPath" />
  </div>
</template>

<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { reactive } from 'vue'

  import { SettingData } from './../type'

  const state = reactive({
    formValues: {} as SettingData
  })

  const openWin = () => {
    invoke('openWin')
  }

  const importSetting = () => {
    invoke('importSetting').then((res: string) => {
      state.formValues = JSON.parse(res)
    })
  }

  const saveHandler = () => {
    console.log(state.formValues)

    invoke('saveSetting', { settingData: JSON.stringify(state.formValues) })
  }

  const exportSetting = () => {}

  const greet = () => {
    invoke('greet')
  }
</script>

<style scoped></style>
