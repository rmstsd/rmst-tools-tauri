import OpenFolder from './pages/OpenFolder'
import Setting from './pages/Setting'

function App() {
  const hash = location.hash.slice(1)

  return (
    <>
      {hash === 'openFolder' && <OpenFolder />}
      {hash === 'setting' && <Setting />}
    </>
  )
}

export default App
