<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { message as antdMessage, Button, Flex, Input, Typography } from 'ant-design-vue'
import { computed, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'

import { useTauriListen } from '@/composables/useTauriListen'
import { LISTEN_KEY } from '@/constants'
import { useChatStore } from '@/stores/chat'
import { useGeneralStore } from '@/stores/general'

const chatStore = useChatStore()
const generalStore = useGeneralStore()
const appWindow = getCurrentWebviewWindow()
const { t } = useI18n()
const { TextArea } = Input
const { Paragraph, Text, Title } = Typography

const placeholder = computed(() => t('pages.chat.inputPlaceholder'))

watch(() => generalStore.appearance.language, () => {
  appWindow.setTitle(t('pages.chat.title'))
}, { immediate: true })

watch(() => chatStore.error, (value) => {
  if (!value) return

  antdMessage.error(value)
}, { flush: 'post' })

useTauriListen<{ content: string }>(LISTEN_KEY.CHAT_STREAM_START, () => {
  chatStore.startStreamingReply()
})

useTauriListen<{ content: string }>(LISTEN_KEY.CHAT_STREAM_CHUNK, ({ payload }) => {
  chatStore.appendStreamingChunk(payload.content)
})

useTauriListen<{ content: string }>(LISTEN_KEY.CHAT_STREAM_END, ({ payload }) => {
  chatStore.finishStreamingReply(payload.content)
})

useTauriListen<{ content: string }>(LISTEN_KEY.CHAT_STREAM_ERROR, ({ payload }) => {
  chatStore.failStreamingReply(payload.content)
})

onMounted(() => {
  if (!chatStore.messages.length) {
    chatStore.messages.push({
      id: crypto.randomUUID(),
      role: 'assistant',
      content: t('pages.chat.welcome'),
    })
  }
})

function onPressEnter(event: KeyboardEvent) {
  if (event.shiftKey) return

  event.preventDefault()
  chatStore.sendMessage()
}
</script>

<template>
  <Flex
    class="h-screen overflow-hidden"
    vertical
  >
    <Flex
      align="center"
      class="border-color-2 dark:border-color-6 border-b bg-color-8 dark:(bg-color-2) px-4 py-3"
      data-tauri-drag-region
      justify="space-between"
    >
      <Flex vertical>
        <Title
          class="m-0!"
          :level="4"
        >
          {{ $t('pages.chat.title') }}
        </Title>
        <Text type="secondary">
          {{ $t('pages.chat.subtitle') }}
        </Text>
      </Flex>

      <Button @click="chatStore.clearConversation">
        {{ $t('pages.chat.clear') }}
      </Button>
    </Flex>

    <Flex
      class="min-h-0 flex-1 overflow-hidden bg-color-8 dark:bg-color-1 p-4"
      gap="small"
      vertical
    >
      <Flex
        v-if="!chatStore.messages.length"
        class="h-full items-center justify-center"
      >
        <Text type="secondary">
          {{ $t('pages.chat.empty') }}
        </Text>
      </Flex>

      <Flex
        v-else
        class="min-h-0 flex-1 overflow-auto"
        gap="middle"
        vertical
      >
        <Flex
          v-for="item in chatStore.messages"
          :key="item.id"
          :justify="item.role === 'user' ? 'end' : 'start'"
        >
          <div
            class="max-w-[85%] rounded-2xl px-4 py-3 text-sm leading-6"
            :class="item.role === 'user'
              ? 'bg-primary-5 text-white'
              : 'bg-color-3 text-color-1 dark:bg-color-3 dark:text-white'"
          >
            <Paragraph class="whitespace-pre-wrap break-words m-0!">
              {{ item.content || (chatStore.isSending && item.role === 'assistant' ? $t('pages.chat.streaming') : '') }}
            </Paragraph>
          </div>
        </Flex>
      </Flex>

      <Flex
        class="border-color-2 dark:border-color-6 border-t pt-4"
        gap="small"
        vertical
      >
        <TextArea
          v-model:value="chatStore.draft"
          :auto-size="{ minRows: 4, maxRows: 8 }"
          :placeholder="placeholder"
          @press-enter="onPressEnter"
        />

        <Flex
          align="center"
          justify="space-between"
        >
          <Text type="secondary">
            {{ chatStore.isSending ? $t('pages.chat.sending') : $t('pages.chat.tip') }}
          </Text>

          <Button
            :disabled="!chatStore.canSend"
            :loading="chatStore.isSending"
            type="primary"
            @click="chatStore.sendMessage"
          >
            {{ $t('pages.chat.send') }}
          </Button>
        </Flex>
      </Flex>
    </Flex>
  </Flex>
</template>
