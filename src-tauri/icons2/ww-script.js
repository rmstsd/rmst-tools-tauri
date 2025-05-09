window.addEventListener('load', () => {
  const title = document.title

  let icon = ''
  window.__TAURI_INTERNALS__.invoke('page_loaded', { title, icon })
  const iconElements = document.querySelectorAll('link[rel*="icon"]')
  if (iconElements.length > 0) {
    icon = iconElements[0].href
  }

  // 获取 title 元素
  const titleElement = document.querySelector('title')

  // 如果没有找到 title 元素，则退出
  if (!titleElement) {
    console.warn('未找到 title 元素')
    return
  }

  // 创建一个 MutationObserver 实例
  const observer = new MutationObserver(mutationsList => {
    for (const mutation of mutationsList) {
      if (mutation.type === 'childList') {
        // 当 title 元素的子节点发生变化时触发
        const newTitle = document.title

        window.__TAURI_INTERNALS__.invoke('page_loaded', { title: newTitle, icon })
      }
    }
  })

  // 配置观察选项
  const config = { childList: true }
  // 开始观察 title 元素
  observer.observe(titleElement, config)
})
