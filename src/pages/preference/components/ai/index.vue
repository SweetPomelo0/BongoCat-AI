<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { Button, Input, message, Select, Switch } from 'ant-design-vue'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

import ProList from '@/components/pro-list/index.vue'
import ProListItem from '@/components/pro-list-item/index.vue'
import { INVOKE_KEY } from '@/constants'
import { useAiStore } from '@/stores/ai'

const aiStore = useAiStore()
const { Password } = Input
const { t } = useI18n()
const isOpeningMemory = ref(false)
const isLoadingMemoryStatus = ref(false)

async function loadMemoryStatus() {
  isLoadingMemoryStatus.value = true

  try {
    const status = await invoke<typeof aiStore.memoryStatus>(INVOKE_KEY.GET_MEMORY_STATUS)

    Object.assign(aiStore.memoryStatus, status)
  } catch (error) {
    message.error(String(error))
  } finally {
    isLoadingMemoryStatus.value = false
  }
}

async function openMemoryPath(target: 'rootDir' | 'personaPath' | 'memoryPath' | 'todayPath') {
  isOpeningMemory.value = true

  try {
    await invoke(INVOKE_KEY.OPEN_MEMORY_PATH, { payload: { target } })
  } catch (error) {
    message.error(String(error))
  } finally {
    isOpeningMemory.value = false
  }
}

onMounted(loadMemoryStatus)
</script>

<template>
  <ProList :title="$t('pages.preference.ai.labels.providerSettings')">
    <ProListItem :title="$t('pages.preference.ai.labels.provider')">
      <Select
        v-model:value="aiStore.config.provider"
        class="w-60"
      >
        <Select.Option value="siliconflow">
          SiliconFlow
        </Select.Option>
        <Select.Option value="custom">
          Custom
        </Select.Option>
      </Select>
    </ProListItem>

    <ProListItem :title="$t('pages.preference.ai.labels.model')">
      <Input
        v-model:value="aiStore.config.model"
        class="w-80"
      />
    </ProListItem>

    <ProListItem
      :description="$t('pages.preference.ai.hints.baseUrl')"
      :title="$t('pages.preference.ai.labels.baseUrl')"
    >
      <Input
        v-model:value="aiStore.config.baseUrl"
        class="w-100"
      />
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.ai.labels.credentialsSettings')">
    <ProListItem
      :description="$t('pages.preference.ai.hints.apiKey')"
      :title="$t('pages.preference.ai.labels.apiKey')"
      vertical
    >
      <Password
        v-model:value="aiStore.config.apiKey"
        class="w-full"
      />
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.ai.labels.memorySettings')">
    <ProListItem
      :description="$t('pages.preference.ai.hints.memoryEnabled')"
      :title="$t('pages.preference.ai.labels.memoryEnabled')"
    >
      <Switch v-model:checked="aiStore.config.memoryEnabled" />
    </ProListItem>

    <ProListItem
      :description="$t('pages.preference.ai.hints.memoryTransparent')"
      :title="$t('pages.preference.ai.labels.memoryFiles')"
      vertical
    >
      <div class="mb-3 flex flex-col gap-1 text-sm dark:text-color-2 text-color-3">
        <span>{{ t('pages.preference.ai.labels.memoryEnabled') }}：{{ aiStore.config.memoryEnabled ? t('pages.preference.ai.status.enabled') : t('pages.preference.ai.status.disabled') }}</span>
        <span>{{ t('pages.preference.ai.labels.memoryRootDir') }}：{{ aiStore.memoryStatus.rootDir || '-' }}</span>
        <span>{{ t('pages.preference.ai.labels.todayNotePath') }}：{{ aiStore.memoryStatus.todayPath || '-' }}</span>
        <span>{{ t('pages.preference.ai.hints.memoryLocalFiles') }}</span>
      </div>

      <div class="flex flex-wrap gap-2">
        <Button
          :loading="isOpeningMemory || isLoadingMemoryStatus"
          @click="openMemoryPath('rootDir')"
        >
          {{ t('pages.preference.ai.buttons.openMemoryFolder') }}
        </Button>
        <Button
          :loading="isOpeningMemory || isLoadingMemoryStatus"
          @click="openMemoryPath('personaPath')"
        >
          {{ t('pages.preference.ai.buttons.openPersona') }}
        </Button>
        <Button
          :loading="isOpeningMemory || isLoadingMemoryStatus"
          @click="openMemoryPath('memoryPath')"
        >
          {{ t('pages.preference.ai.buttons.openMemory') }}
        </Button>
        <Button
          :loading="isOpeningMemory || isLoadingMemoryStatus"
          @click="openMemoryPath('todayPath')"
        >
          {{ t('pages.preference.ai.buttons.openTodayNote') }}
        </Button>
        <Button
          :loading="isLoadingMemoryStatus"
          @click="loadMemoryStatus"
        >
          {{ t('pages.preference.ai.buttons.refreshMemoryStatus') }}
        </Button>
      </div>
    </ProListItem>
  </ProList>
</template>
