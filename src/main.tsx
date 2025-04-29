import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'

import '@arco-design/web-react/dist/css/arco.css'
import './main.less'
import { ConfigProvider } from '@arco-design/web-react'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <ConfigProvider componentConfig={{ Button: { type: 'primary' } }}>
    <App />
  </ConfigProvider>
)
