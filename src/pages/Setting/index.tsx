import { Tabs } from '@arco-design/web-react'
import { useEffect, useState } from 'react'
import SettingConfig from './SettingConfig'
import SmallTool from './SmallTool'
import QrCode from './QrCode'

export default function SettingPage() {
  const [activeKey, setActiveKey] = useState(localStorage.getItem('activeKey') || '1')
  const [qrCodeValue, setQrCodeValue] = useState('rmst')

  useEffect(() => {}, [])

  return (
    <Tabs
      tabPosition="left"
      activeTab={activeKey}
      onChange={k => {
        localStorage.setItem('activeKey', k)
        setActiveKey(k)
      }}
      destroyOnHide
      className="setting-page-root h-screen"
    >
      <Tabs.TabPane key="1" title="设置" className="h-screen overflow-auto">
        <SettingConfig />
      </Tabs.TabPane>
      <Tabs.TabPane key="2" title="工具" className="h-screen overflow-auto">
        <SmallTool />
      </Tabs.TabPane>
      <Tabs.TabPane key="3" title="二维码" className="h-screen overflow-auto">
        <QrCode value={qrCodeValue} setValue={setQrCodeValue} />
      </Tabs.TabPane>
    </Tabs>
  )
}
