import { PhysicalPosition } from '@tauri-apps/api/dpi'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { onMounted, ref, watch } from 'vue'

import { useCatStore } from '@/stores/cat'
import { getCursorMonitor } from '@/utils/monitor'

const appWindow = getCurrentWebviewWindow()

export function useWindowPosition() {
  const catStore = useCatStore()
  const isMounted = ref(false)
  const scaleChangeCount = ref(0)

  const setWindowPosition = async () => {
    const monitor = await getCursorMonitor()

    if (!monitor) return

    const windowSize = await appWindow.outerSize()
    const { position, size } = monitor

    switch (catStore.window.position) {
      case 'topLeft':
        return appWindow.setPosition(new PhysicalPosition(position.x, position.y))
      case 'topRight':
        return appWindow.setPosition(new PhysicalPosition(position.x + size.width - windowSize.width, position.y))
      case 'bottomLeft':
        return appWindow.setPosition(new PhysicalPosition(position.x, position.y + size.height - windowSize.height))
      default:
        return appWindow.setPosition(new PhysicalPosition(position.x + size.width - windowSize.width, position.y + size.height - windowSize.height))
    }
  }

  onMounted(() => {
    isMounted.value = true

    appWindow.onScaleChanged(() => {
      scaleChangeCount.value += 1
      setWindowPosition()
    })
  })

  watch(() => catStore.window.position, setWindowPosition)

  return {
    isMounted,
    scaleChangeCount,
    setWindowPosition,
  }
}
