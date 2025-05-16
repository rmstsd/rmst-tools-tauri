import { SettingData } from '../../type'
import { invoke } from '@tauri-apps/api/core'
import { info as logInfo, error as logError } from '@tauri-apps/plugin-log'
import { Button, Divider, Form, Input, Link, Message, Modal, Switch, Tag, Typography } from '@arco-design/web-react'
import { IconDelete } from '@arco-design/web-react/icon'
import { useEffect, useState } from 'react'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

const format = (dateTime: string) => {
  return new Intl.DateTimeFormat('zh', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  })
    .format(new Date(dateTime))
    .replace(/[/]/g, '-')
}

export default function Setting() {
  const [form] = Form.useForm<SettingData>()

  const [appInfo, setAppInfo] = useState({})
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    getSettingData()

    invoke('get_package_info').then(data => {
      console.log(data)
      setAppInfo(data)
    })
  }, [])

  const getSettingData = () => {
    form.resetFields()

    invoke('getSetting').then((data: SettingData) => {
      form.setFieldsValue(data)
    })
  }

  const importSetting = () => {
    invoke('importSetting').then(() => {
      Message.success({ content: '操作成功', position: 'bottom' })
      getSettingData()
    })
  }

  const saveHandler = () => {
    const formValues = form.getFieldsValue()
    invoke('saveSetting', { settingData: formValues }).then(() => {
      Message.success({ content: '操作成功', position: 'bottom' })
    })
  }

  const exportSetting = () => {
    invoke('exportSetting').then(() => {
      Message.success({ content: '操作成功', position: 'bottom' })
    })
  }

  const clearStore = () => {
    invoke('clearStore').then(() => {
      Message.success({ content: '操作成功', position: 'bottom' })
      getSettingData()
    })
  }

  const checkUpdate = async () => {
    setLoading(true)
    try {
      const info = await check()

      if (info) {
        console.log('有新版本', info)
        Modal.confirm({
          title: '发现新版本',
          content: (
            <div>
              <Typography.Title heading={5}>下载吗?</Typography.Title>

              <div className="text-xl">
                <Typography.Title heading={6}>新版本: {info.version}</Typography.Title>
                <div>当前版本: {info.currentVersion}</div>
                <div>发布时间: {format(info.date)}</div>
              </div>
            </div>
          ),
          async onOk() {
            console.log('确定 下载')
            logInfo('确定 下载')

            await info.downloadAndInstall(progress => {
              console.log(progress)
            })

            relaunch()

            return

            info.download(progress => {
              console.log('progress', progress)

              if (progress.event === 'Finished') {
                Modal.confirm({
                  title: '下载完成',
                  content: '更新吗?',
                  async onOk() {
                    logInfo('确定 更新')

                    info.install().then(() => {
                      logInfo('更新完成')
                      console.log('更新完成')
                      relaunch()
                    })
                  }
                })
              }
            })
          }
        })
      } else {
        console.log('没有新版本')

        Message.info({ content: '没有新版本' })
      }
    } catch (err: any) {
      console.log(err)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div>
      <Form className="pr-[10%]" form={form} autoComplete="off">
        <div className="flex flex-wrap gap-3 my-2" style={{ fontSize: 16 }}>
          {Object.keys(appInfo).map(k => (
            <div key={k} className="flex gap-2">
              <div>{k}:</div>
              <Tag size="medium">{appInfo[k]}</Tag>
            </div>
          ))}
        </div>

        <Form.Item label=" " className="sticky top-0 z-10 mt-2 bg-white border-b pb-2 pt-2">
          <div className="flex flex-wrap items-center gap-x-3">
            <Button
              onClick={() => {
                logError('测试 log')
              }}
            >
              log test
            </Button>
            <Button
              onClick={() => {
                relaunch()
              }}
            >
              relaunch
            </Button>
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

            <Button onClick={checkUpdate} loading={loading}>
              检查更新
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
    </div>
  )
}
