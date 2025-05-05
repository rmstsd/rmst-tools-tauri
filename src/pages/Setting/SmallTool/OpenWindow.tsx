import { Button, Form, Input, Link, Typography } from '@arco-design/web-react'
import { useState } from 'react'
import { useInterval } from 'ahooks'
import { invoke } from '@tauri-apps/api/core'
import { getAllWebviewWindows } from '@tauri-apps/api/webviewWindow'

const OpenWindow = () => {
  const [form] = Form.useForm()
  const [historyOpenedUrls, setHistoryOpenedUrls] = useState([])

  useInterval(
    () => {
      updateOpenedUrls()
    },
    2000,
    { immediate: true }
  )

  const updateOpenedUrls = async () => {
    const aa = await getAllWebviewWindows()
    console.log(aa)
    const urls: string[] = await invoke('getHistoryOpenedUrls')

    setHistoryOpenedUrls(urls)
  }

  const Open_Url_Win = (url: string) => {
    invoke('openWin', { url }).then(() => {
      updateOpenedUrls()
    })
  }

  return (
    <div style={{ paddingRight: '10%' }}>
      <Form form={form} initialValues={{ url: 'https://www.bilibili.com/' }}>
        <Form.Item label="opened url" field="url" rules={[{ required: true, type: 'url' }]}>
          <Input placeholder="http://www.example.com" />
        </Form.Item>
        <Form.Item label=" ">
          <Button
            onClick={async () => {
              await form.validate()
              Open_Url_Win(form.getFieldValue('url'))
              form.resetFields()
            }}
          >
            打开
          </Button>
          <Button
            onClick={async () => {
              await invoke('clearHistoryOpenedUrls')
              updateOpenedUrls()
            }}
            status="danger"
            className="ml-3"
          >
            清空历史
          </Button>
        </Form.Item>
        <Form.Item label="历史打开">
          {historyOpenedUrls.map((item, idx) => (
            <div className="flex items-center gap-[10px] my-2" key={idx}>
              <Link onClick={() => Open_Url_Win(item)}>open</Link>
              <Typography.Ellipsis rows={1} className="flex-grow w-0">
                {item}
              </Typography.Ellipsis>
            </div>
          ))}
        </Form.Item>
      </Form>
    </div>
  )
}

export default OpenWindow
