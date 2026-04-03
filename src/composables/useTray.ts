import type { TrayIconOptions } from '@tauri-apps/api/tray'

import { getName, getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'
import { Menu, MenuItem, PredefinedMenuItem } from '@tauri-apps/api/menu'
import { resolveResource } from '@tauri-apps/api/path'
import { TrayIcon } from '@tauri-apps/api/tray'
import { openUrl } from '@tauri-apps/plugin-opener'
import { watchDebounced } from '@vueuse/core'
import { watch } from 'vue'
import { useI18n } from 'vue-i18n'

import { GITHUB_LINK, INVOKE_KEY, LISTEN_KEY } from '../constants'
import { showWindow } from '../plugins/window'
import { isMac } from '../utils/platform'

import { useSharedMenu } from './useSharedMenu'

import { useCatStore } from '@/stores/cat'
import { useGeneralStore } from '@/stores/general'

const TRAY_ID = 'BONGO_CAT_TRAY'

export function useTray() {
  const catStore = useCatStore()
  const generalStore = useGeneralStore()
  const { getSharedMenu } = useSharedMenu()
  const { t } = useI18n()

  watch([() => catStore.window.visible, () => catStore.window.passThrough, () => generalStore.appearance.language], () => {
    updateTrayMenu()
  })

  watchDebounced([() => catStore.window.scale, () => catStore.window.opacity], () => {
    updateTrayMenu()
  }, { debounce: 200 })

  async function handleRestartApp() {
    await invoke(INVOKE_KEY.RESTART_APP)
  }

  async function handleQuitApp() {
    await invoke(INVOKE_KEY.QUIT_APP)
  }

  const createTray = async () => {
    const tray = await getTrayById()

    if (tray) return

    const appName = await getName()
    const appVersion = await getVersion()

    const menu = await getTrayMenu()

    const path = isMac ? 'assets/tray-mac.png' : 'assets/tray.png'
    const icon = await resolveResource(path)

    const options: TrayIconOptions = {
      menu,
      icon,
      id: TRAY_ID,
      tooltip: `${appName} v${appVersion}`,
      iconAsTemplate: true,
      menuOnLeftClick: true,
    }

    return TrayIcon.new(options)
  }

  const getTrayById = () => {
    return TrayIcon.getById(TRAY_ID)
  }

  const getTrayMenu = async () => {
    const appVersion = await getVersion()

    const items = await Promise.all([
      ...await getSharedMenu(),
      PredefinedMenuItem.new({ item: 'Separator' }),
      MenuItem.new({
        text: t('composables.useTray.checkUpdate'),
        action: () => {
          showWindow()

          emit(LISTEN_KEY.UPDATE_APP)
        },
      }),
      MenuItem.new({
        text: t('composables.useTray.openSource'),
        action: () => openUrl(GITHUB_LINK),
      }),
      PredefinedMenuItem.new({ item: 'Separator' }),
      MenuItem.new({
        text: `v${appVersion}`,
        enabled: false,
      }),
      MenuItem.new({
        text: t('composables.useTray.restartApp'),
        action: handleRestartApp,
      }),
      MenuItem.new({
        text: t('composables.useTray.quitApp'),
        accelerator: isMac ? 'Cmd+Q' : '',
        action: handleQuitApp,
      }),
    ])

    return Menu.new({ items })
  }

  const updateTrayMenu = async () => {
    const tray = await getTrayById()

    if (!tray) return

    const menu = await getTrayMenu()

    tray.setMenu(menu)
  }

  return {
    createTray,
  }
}
