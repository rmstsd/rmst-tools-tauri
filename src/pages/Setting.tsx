import { SettingData } from '../type'
import { invoke } from '@tauri-apps/api/core'
import { Button, Divider, Form, Input, Link, Message, Switch, Tag, Typography } from '@arco-design/web-react'
import { IconDelete } from '@arco-design/web-react/icon'
import { useEffect, useState } from 'react'

export default function Setting() {
  const [form] = Form.useForm<SettingData>()

  useEffect(() => {
    getSettingData()
  }, [])

  const getSettingData = () => {
    form.resetFields()

    invoke('getSetting').then((data: SettingData) => {
      form.setFieldsValue(data)
    })
  }

  const importSetting = () => {
    invoke('importSetting').then(() => {
      getSettingData()
    })
  }

  const saveHandler = () => {
    const formValues = form.getFieldsValue()
    invoke('saveSetting', { settingData: formValues })
  }

  const exportSetting = () => {
    invoke('exportSetting')
  }

  const greet = () => {
    invoke('greet')
  }

  const clearStore = () => {
    invoke('clearStore').then(() => {
      getSettingData()
    })
  }

  return (
    <div>
      <Form className="pr-[10%]" form={form} autoComplete="off">
        <Form.Item label=" " className="sticky top-0 z-10 mt-2 bg-white border-b pb-2 pt-2">
          <div className="flex flex-wrap items-center gap-x-3">
            <h2>设置</h2>
            <Button type="primary" onClick={saveHandler}>
              保存
            </Button>
            <Button onClick={getSettingData}>刷新</Button>
            <Button.Group>
              <Button type="outline" onClick={exportSetting}>
                导出
              </Button>
              <Button type="outline" onClick={importSetting}>
                导入
              </Button>
            </Button.Group>
            <Button type="primary" status="danger" onClick={clearStore}>
              清空本地缓存
            </Button>
          </div>
        </Form.Item>

        <Form.Item label="编辑器路径列表">
          <Form.List field="editorPaths">
            {(fields, { add, remove }) => {
              return (
                <div>
                  {fields.map((item, index) => {
                    return (
                      <div key={item.key} className="flex gap-[10px]">
                        <Form.Item field={`${item.field}`} className="flex-grow">
                          <Input placeholder="例如: D:\Microsoft VS Code\Code.exe" />
                        </Form.Item>
                        <Button
                          className="shrink-0"
                          onClick={() => remove(index)}
                          shape="circle"
                          status="danger"
                          icon={<IconDelete />}
                        ></Button>
                      </div>
                    )
                  })}
                  <div>
                    <Button onClick={() => add()}>Add</Button>
                  </div>
                </div>
              )
            }}
          </Form.List>
        </Form.Item>

        <Form.Item label="cmd Path" field="cmdPath">
          <Input placeholder="例如: D:\WindowsTerminal\wt.exe" />
        </Form.Item>

        <Form.Item label="项目目录列表">
          <Form.List field="projectPaths">
            {(fields, { add, remove }) => {
              return (
                <div>
                  {fields.map((item, index) => {
                    return (
                      <div key={item.key} className="flex gap-[10px]">
                        <Form.Item field={item.field}>
                          <Input placeholder="例如: E:\project" />
                        </Form.Item>
                        <Button
                          icon={<IconDelete />}
                          shape="circle"
                          status="danger"
                          onClick={() => remove(index)}
                          className="shrink-0"
                        />
                      </div>
                    )
                  })}
                  <Button onClick={() => add()}>add</Button>
                </div>
              )
            }}
          </Form.List>
        </Form.Item>

        <Form.Item label="笔记列表">
          <Form.List field="notes">
            {(fields, { add, remove }) => {
              return (
                <div>
                  {fields.map((item, index) => {
                    return (
                      <div key={item.key} className="flex gap-[10px]">
                        <Form.Item field={item.field}>
                          <Input placeholder="任意字符串" />
                        </Form.Item>
                        <Button
                          icon={<IconDelete />}
                          shape="circle"
                          status="danger"
                          className="shrink-0"
                          onClick={() => remove(index)}
                        />
                      </div>
                    )
                  })}

                  <Button onClick={() => add()}>add</Button>
                </div>
              )
            }}
          </Form.List>
        </Form.Item>
      </Form>

      <Divider />

      <OpenWin />
    </div>
  )
}

import { useInterval } from 'ahooks'
const OpenWin = () => {
  const [form] = Form.useForm()
  const [historyOpenedUrls, setHistoryOpenedUrls] = useState([])

  useInterval(
    () => {
      // updateOpenedUrls()
    },
    2000,
    { immediate: true }
  )

  const updateOpenedUrls = async () => {
    const urls: string[] = await invoke('getHistoryOpenedUrls')

    console.log('urls', urls)
    // setHistoryOpenedUrls(urls)
  }

  const Open_Url_Win = (url: string) => {
    invoke('openWin', { url })
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
