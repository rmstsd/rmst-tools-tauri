import { Tabs } from '@arco-design/web-react'
import SettingConfig from './SettingConfig'
import SmallTool from './SmallTool'
import { useEffect, useState } from 'react'

export default function SettingPage() {
  const [activeKey, setActiveKey] = useState('1')
  const [qrCodeValue, setQrCodeValue] = useState('rmst')

  useEffect(() => {}, [])

  return (
    <Tabs
      tabPosition="left"
      activeTab={activeKey}
      onChange={setActiveKey}
      destroyOnHide
      className="setting-page-root h-screen"
    >
      <Tabs.TabPane key="1" title="设置" className="h-screen overflow-auto">
        <SettingConfig />
      </Tabs.TabPane>
      <Tabs.TabPane key="2" title="工具" className="h-screen overflow-auto">
        <SmallTool />
      </Tabs.TabPane>
      {/* <Tabs.TabPane key="3" title="二维码" className="h-screen overflow-auto">
        <QrCode value={qrCodeValue} setValue={setQrCodeValue} />
      </Tabs.TabPane> */}
    </Tabs>
  )
}
