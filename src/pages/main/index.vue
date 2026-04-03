<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import { PhysicalSize } from '@tauri-apps/api/dpi'
import { Menu } from '@tauri-apps/api/menu'
import { sep } from '@tauri-apps/api/path'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { exists, readDir } from '@tauri-apps/plugin-fs'
import { useDebounceFn, useEventListener } from '@vueuse/core'
import { round } from 'es-toolkit'
import { nth } from 'es-toolkit/compat'
import { onMounted, onUnmounted, ref, watch } from 'vue'

import { useDevice } from '@/composables/useDevice'
import { useGamepad } from '@/composables/useGamepad'
import { useModel } from '@/composables/useModel'
import { useSharedMenu } from '@/composables/useSharedMenu'
import { useWindowPosition } from '@/composables/useWindowPosition'
import { hideWindow, setAlwaysOnTop, setTaskbarVisibility, showWindow } from '@/plugins/window'
import { useCatStore } from '@/stores/cat'
import { useGeneralStore } from '@/stores/general.ts'
import { useModelStore } from '@/stores/model'
import { isImage } from '@/utils/is'
import { join } from '@/utils/path'
import { clearObject } from '@/utils/shared'

const MIN_WINDOW_SIZE = 240
const MIN_SCALE = 50

const { startListening } = useDevice()
const appWindow = getCurrentWebviewWindow()
const { modelSize, handleLoad, handleDestroy, handleResize, handleKeyChange } = useModel()
const catStore = useCatStore()
const { getSharedMenu } = useSharedMenu()
const modelStore = useModelStore()
const generalStore = useGeneralStore()
const resizing = ref(false)
const backgroundImagePath = ref<string>()
const { stickActive } = useGamepad()
const { isMounted, scaleChangeCount, setWindowPosition } = useWindowPosition()
const isApplyingWindowSize = ref(false)
const isCursorInsideWindow = ref(false)

onMounted(startListening)

onUnmounted(handleDestroy)

async function applyModelLayout(reposition = true) {
  await handleResize(innerWidth, innerHeight)

  if (reposition) {
    await setWindowPosition()
  }
}

const debouncedResize = useDebounceFn(async () => {
  await applyModelLayout(!isApplyingWindowSize.value)

  resizing.value = false
}, 100)

useEventListener('resize', () => {
  resizing.value = true

  debouncedResize()
})

watch(() => modelStore.currentModel, async (model) => {
  if (!model) return

  const loaded = await handleLoad()

  if (!loaded) return

  const path = join(model.path, 'resources', 'background.png')

  const existed = await exists(path)

  backgroundImagePath.value = existed ? convertFileSrc(path) : void 0

  clearObject([modelStore.supportKeys, modelStore.pressedKeys])

  const resourcePath = join(model.path, 'resources')
  const groups = ['left-keys', 'right-keys']

  for await (const groupName of groups) {
    const groupDir = join(resourcePath, groupName)
    const files = await readDir(groupDir).catch(() => [])
    const imageFiles = files.filter(file => isImage(file.name))

    for (const file of imageFiles) {
      const fileName = file.name.split('.')[0]

      modelStore.supportKeys[fileName] = join(groupDir, file.name)
    }
  }

  await applyModelLayout()
}, { deep: true, immediate: true })

watch([() => catStore.window.scale, modelSize], async ([scale, modelSize]) => {
  if (!modelSize) return

  const { width, height } = modelSize
  const targetWidth = Math.max(Math.round(width * (scale / 100)), MIN_WINDOW_SIZE)
  const targetHeight = Math.max(Math.round(height * (scale / 100)), MIN_WINDOW_SIZE)
  const currentSize = await appWindow.innerSize()

  if (currentSize.width === targetWidth && currentSize.height === targetHeight) {
    await applyModelLayout(false)
    return
  }

  isApplyingWindowSize.value = true

  try {
    await appWindow.setSize(new PhysicalSize({
      width: targetWidth,
      height: targetHeight,
    }))
  } finally {
    isApplyingWindowSize.value = false
  }

  await applyModelLayout()
}, { immediate: true })

watch(scaleChangeCount, async () => {
  await applyModelLayout()
})

watch([modelStore.pressedKeys, stickActive], ([keys, stickActive]) => {
  const dirs = Object.values(keys).map((path) => {
    return nth(path.split(sep()), -2)!
  })

  const hasLeft = dirs.some(dir => dir.startsWith('left'))
  const hasRight = dirs.some(dir => dir.startsWith('right'))

  handleKeyChange(true, stickActive.left || hasLeft)
  handleKeyChange(false, stickActive.right || hasRight)
}, { deep: true })

watch(() => catStore.window.visible, async (value) => {
  value ? showWindow() : hideWindow()
})

watch([() => catStore.window.passThrough, () => catStore.window.hideOnHover, isCursorInsideWindow], ([passThrough, hideOnHover, cursorInsideWindow]) => {
  appWindow.setIgnoreCursorEvents(passThrough || (hideOnHover && cursorInsideWindow))
}, { immediate: true })

watch(() => generalStore.app.taskbarVisible, setTaskbarVisibility, { immediate: true })
watch(() => catStore.window.alwaysOnTop, setAlwaysOnTop, { immediate: true })

function handleMouseDown() {
  appWindow.startDragging()
}

async function handleContextmenu(event: MouseEvent) {
  event.preventDefault()

  if (event.shiftKey) return

  const menu = await Menu.new({
    items: await getSharedMenu(),
  })

  menu.popup()
}

function handleMouseMove(event: MouseEvent) {
  isCursorInsideWindow.value = true

  const { buttons, shiftKey, movementX, movementY } = event

  if (buttons !== 2 || !shiftKey) return

  const delta = (movementX + movementY) * 0.5
  const nextScale = Math.max(MIN_SCALE, Math.min(catStore.window.scale + delta, 500))

  catStore.window.scale = round(nextScale)
}

function handleMouseLeave() {
  isCursorInsideWindow.value = false
}
</script>

<template>
  <div
    v-show="isMounted"
    class="relative size-full overflow-hidden children:(absolute size-full)"
    :class="{ '-scale-x-100': catStore.model.mirror }"
    :style="{
      opacity: catStore.window.opacity / 100,
      borderRadius: `${catStore.window.radius}%`,
    }"
    @contextmenu="handleContextmenu"
    @mousedown="handleMouseDown"
    @mouseleave="handleMouseLeave"
    @mousemove="handleMouseMove"
  >
    <img
      v-if="backgroundImagePath"
      class="object-cover"
      :src="backgroundImagePath"
    >

    <canvas id="live2dCanvas" />

    <img
      v-for="path in modelStore.pressedKeys"
      :key="path"
      class="object-contain"
      :src="convertFileSrc(path)"
    >

    <div
      v-show="resizing"
      class="flex items-center justify-center bg-black"
    >
      <span class="text-center text-[10vw] text-white">
        {{ $t('pages.main.hints.redrawing') }}
      </span>
    </div>
  </div>
</template>
