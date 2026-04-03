import { defineStore } from 'pinia'
import { reactive } from 'vue'

export interface CatStore {
  model: {
    mirror: boolean
    single: boolean
    mouseMirror: boolean
    autoReleaseDelay: number
  }
  window: {
    visible: boolean
    passThrough: boolean
    alwaysOnTop: boolean
    scale: number
    opacity: number
    radius: number
    hideOnHover: boolean
    position: 'topLeft' | 'topRight' | 'bottomLeft' | 'bottomRight'
  }
}

export const useCatStore = defineStore('cat', () => {
  const model = reactive<CatStore['model']>({
    mirror: false,
    single: false,
    mouseMirror: false,
    autoReleaseDelay: 3,
  })

  const window = reactive<CatStore['window']>({
    visible: true,
    passThrough: false,
    alwaysOnTop: false,
    scale: 100,
    opacity: 100,
    radius: 0,
    hideOnHover: false,
    position: 'bottomRight',
  })

  const init = () => {
    window.visible = true
  }

  return {
    model,
    window,
    init,
  }
})
