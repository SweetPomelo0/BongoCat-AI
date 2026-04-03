import { defineStore } from 'pinia'
import { reactive } from 'vue'

export interface AiStore {
  provider: 'siliconflow' | 'custom'
  model: string
  apiKey: string
  baseUrl: string
  memoryEnabled: boolean
}

export interface MemoryStatus {
  rootDir: string
  personaPath: string
  memoryPath: string
  todayPath: string
}

export const useAiStore = defineStore('ai', () => {
  const config = reactive<AiStore>({
    provider: 'siliconflow',
    model: 'Qwen/Qwen2.5-7B-Instruct',
    apiKey: '',
    baseUrl: 'https://api.siliconflow.cn/v1/chat/completions',
    memoryEnabled: true,
  })

  const memoryStatus = reactive<MemoryStatus>({
    rootDir: '',
    personaPath: '',
    memoryPath: '',
    todayPath: '',
  })

  return {
    config,
    memoryStatus,
  }
})
