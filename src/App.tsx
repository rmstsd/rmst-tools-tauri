import { useState } from 'react'
import reactLogo from './assets/react.svg'
import { invoke } from '@tauri-apps/api/core'
import OpenFolder from './pages/OpenFolder'
import Setting from './pages/Setting'

function App() {
  const [currentPage] = useState('')

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // setGreetMsg(await invoke('greet', { name }))
  }

  return <>{currentPage === 'openFolder' ? <OpenFolder /> : <Setting />}</>
}

export default App
