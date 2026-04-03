import type { PhysicalPosition } from '@tauri-apps/api/dpi'

import { resolveResource, sep } from '@tauri-apps/api/path'
import { message } from 'ant-design-vue'
import { isNil } from 'es-toolkit'
import { nth } from 'es-toolkit/compat'
import { ref } from 'vue'

import live2d from '../utils/live2d'

import { useCatStore } from '@/stores/cat'
import { useModelStore } from '@/stores/model'
import { getCursorMonitor } from '@/utils/monitor'

export interface ModelSize {
  width: number
  height: number
}

export function useModel() {
  const modelStore = useModelStore()
  const catStore = useCatStore()
  const modelSize = ref<ModelSize>()

  async function handleLoad() {
    try {
      if (!modelStore.currentModel) return false

      const { path } = modelStore.currentModel

      await resolveResource(path)

      const { width, height, ...rest } = await live2d.load(path)

      modelSize.value = { width, height }
      Object.assign(modelStore, rest)

      return true
    } catch (error) {
      message.error(String(error))

      return false
    }
  }

  function handleDestroy() {
    live2d.destroy()
  }

  async function handleResize(viewportWidth = innerWidth, viewportHeight = innerHeight) {
    if (!modelSize.value) return

    live2d.resizeModel(modelSize.value, viewportWidth, viewportHeight)
  }

  const handlePress = (key: string) => {
    const path = modelStore.supportKeys[key]

    if (!path) return

    if (catStore.model.single) {
      const dirName = nth(path.split(sep()), -2)!

      const filterKeys = Object.entries(modelStore.pressedKeys).filter(([, value]) => {
        return value.includes(dirName)
      })

      for (const [key] of filterKeys) {
        handleRelease(key)
      }
    }

    modelStore.pressedKeys[key] = path
  }

  const handleRelease = (key: string) => {
    delete modelStore.pressedKeys[key]
  }

  function handleKeyChange(isLeft = true, pressed = true) {
    const id = isLeft ? 'CatParamLeftHandDown' : 'CatParamRightHandDown'

    live2d.setParameterValue(id, pressed)
  }

  function handleMouseChange(key: string, pressed = true) {
    const id = key === 'Left' ? 'ParamMouseLeftDown' : 'ParamMouseRightDown'

    live2d.setParameterValue(id, pressed)
  }

  async function handleMouseMove(cursorPoint: PhysicalPosition) {
    const monitor = await getCursorMonitor(cursorPoint)

    if (!monitor) return

    const { size, position } = monitor

    const xRatio = (cursorPoint.x - position.x) / size.width
    const yRatio = (cursorPoint.y - position.y) / size.height

    for (const id of ['ParamMouseX', 'ParamMouseY', 'ParamAngleX', 'ParamAngleY']) {
      const { min, max } = live2d.getParameterRange(id)

      if (isNil(min) || isNil(max)) continue

      const isXAxis = id.endsWith('X')

      const ratio = isXAxis ? xRatio : yRatio
      let value = max - (ratio * (max - min))

      if (isXAxis && catStore.model.mouseMirror) {
        value *= -1
      }

      live2d.setParameterValue(id, value)
    }
  }

  async function handleAxisChange(id: string, value: number) {
    const { min, max } = live2d.getParameterRange(id)

    live2d.setParameterValue(id, Math.max(min, value * max))
  }

  return {
    modelSize,
    handlePress,
    handleRelease,
    handleLoad,
    handleDestroy,
    handleResize,
    handleKeyChange,
    handleMouseChange,
    handleMouseMove,
    handleAxisChange,
  }
}
