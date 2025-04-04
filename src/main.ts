import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')

import { TrayIcon } from '@tauri-apps/api/tray'
import { defaultWindowIcon } from '@tauri-apps/api/app'

import { Menu } from '@tauri-apps/api/menu'

const onTrayMenuClick = id => {}

const menu = await Menu.new({
  items: [
    {
      id: 'quit',
      text: 'Quit',
      action: onTrayMenuClick
    }
  ]
})

console.log(await defaultWindowIcon())

const options = {
  menu,
  icon: await defaultWindowIcon()
}

const tray = await TrayIcon.new(options)
