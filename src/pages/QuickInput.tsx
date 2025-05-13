import { Button } from '@arco-design/web-react'
import { invoke } from '@tauri-apps/api/core'

export default function QuickInput() {
  return (
    <div>
      <Button
        onClick={() => {
          invoke('CopyAndPaste')
        }}
      >
        dd
      </Button>
    </div>
  )
}
