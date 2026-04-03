import type { Theme } from '@tauri-apps/api/window'

import { defineStore } from 'pinia'
import { getLocale } from 'tauri-plugin-locale-api'
import { reactive } from 'vue'

import { LANGUAGE } from '@/constants'

export type Language = typeof LANGUAGE[keyof typeof LANGUAGE]

export interface GeneralStore {
  app: {
    autostart: boolean
    taskbarVisible: boolean
  }
  appearance: {
    theme: 'auto' | Theme
    isDark: boolean
    language?: Language
  }
  update: {
    autoCheck: boolean
  }
}

export const useGeneralStore = defineStore('general', () => {
  const app = reactive<GeneralStore['app']>({
    autostart: false,
    taskbarVisible: false,
  })

  const appearance = reactive<GeneralStore['appearance']>({
    theme: 'auto',
    isDark: false,
  })

  const update = reactive<GeneralStore['update']>({
    autoCheck: false,
  })

  const getLanguage = async () => {
    const locale = await getLocale<Language>()

    if (Object.values(LANGUAGE).includes(locale)) {
      return locale
    }

    return LANGUAGE.EN_US
  }

  const init = async () => {
    appearance.language ??= await getLanguage()
  }

  return {
    app,
    appearance,
    update,
    init,
  }
})
