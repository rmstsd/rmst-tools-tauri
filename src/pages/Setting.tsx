import { SettingData } from '../type'
import { invoke } from '@tauri-apps/api/core'
import { Button, Form, Input, Message, Switch, Tag } from '@arco-design/web-react'
import { IconDelete } from '@arco-design/web-react/icon'
import { useEffect, useState } from 'react'

export default function Setting() {
  const [formValues, setFormValues] = useState({} as SettingData)
  const [form] = Form.useForm()

  useEffect(() => {}, [])

  const getSettingData = () => {
    invoke('getSetting').then(data => {
      console.log(data)
      const dataObject = JSON.parse(data)
      console.log(dataObject)
      form.setFieldsValue(dataObject)
    })
  }

  const openWin = () => {
    invoke('openWin')
  }

  const importSetting = () => {
    invoke('importSetting').then((res: string) => {
      setFormValues(JSON.parse(res))
    })
  }

  const saveHandler = () => {
    console.log(formValues)

    invoke('saveSetting', { settingData: JSON.stringify(formValues) })
  }

  const exportSetting = () => {
    invoke('exportSetting')
  }

  const greet = () => {
    invoke('greet')
  }

  const clearStore = () => {
    invoke('clearStore')
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
            <Button onClick={getSettingData}>加载设置数据</Button>
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
                        <Form.Item field={`${item.field}.path`} className="flex-grow">
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
    </div>
  )
}
