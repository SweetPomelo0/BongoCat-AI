import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

import { INVOKE_KEY } from '@/constants'
import { useAiStore } from '@/stores/ai'

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant'
  content: string
}

export const useChatStore = defineStore('chat', () => {
  const aiStore = useAiStore()
  const draft = ref('')
  const messages = ref<ChatMessage[]>([])
  const isSending = ref(false)
  const error = ref('')
  const streamingMessageId = ref('')

  const canSend = computed(() => draft.value.trim().length > 0 && !isSending.value)

  function getHistory() {
    return messages.value.map(({ role, content }) => ({ role, content }))
  }

  function startStreamingReply() {
    const id = crypto.randomUUID()
    streamingMessageId.value = id

    messages.value.push({
      id,
      role: 'assistant',
      content: '',
    })
  }

  function appendStreamingChunk(content: string) {
    const current = messages.value.find(item => item.id === streamingMessageId.value)

    if (!current) return

    current.content += content
  }

  function finishStreamingReply(content?: string) {
    const current = messages.value.find(item => item.id === streamingMessageId.value)

    if (current && content) {
      current.content = content
    }

    streamingMessageId.value = ''
    isSending.value = false
  }

  function failStreamingReply(message: string) {
    error.value = message
    const current = messages.value.find(item => item.id === streamingMessageId.value)

    if (current && !current.content) {
      messages.value = messages.value.filter(item => item.id !== streamingMessageId.value)
    }

    streamingMessageId.value = ''
    isSending.value = false
  }

  async function sendMessage() {
    const content = draft.value.trim()

    if (!content || isSending.value) return

    const history = getHistory()

    error.value = ''
    draft.value = ''

    messages.value.push({
      id: crypto.randomUUID(),
      role: 'user',
      content,
    })

    isSending.value = true

    try {
      await invoke(INVOKE_KEY.SEND_CHAT_MESSAGE_STREAM, {
        message: content,
        history,
        config: aiStore.config,
      })
    } catch (err) {
      failStreamingReply(String(err))
    }
  }

  function clearConversation() {
    messages.value = []
    error.value = ''
    streamingMessageId.value = ''
  }

  return {
    draft,
    messages,
    isSending,
    error,
    canSend,
    sendMessage,
    clearConversation,
    startStreamingReply,
    appendStreamingChunk,
    finishStreamingReply,
    failStreamingReply,
  }
})
