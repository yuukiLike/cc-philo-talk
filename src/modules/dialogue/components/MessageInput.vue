<script setup lang="ts">
import { ref } from "vue";

defineProps<{
  disabled: boolean;
}>();

const emit = defineEmits<{
  send: [content: string];
}>();

const input = ref("");

function handleSend() {
  const content = input.value.trim();
  if (!content) return;
  emit("send", content);
  input.value = "";
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSend();
  }
}
</script>

<template>
  <div class="px-6 py-4 border-t border-stone-200 bg-white">
    <div class="flex gap-3 items-end">
      <textarea
        v-model="input"
        @keydown="handleKeydown"
        :disabled="disabled"
        rows="2"
        class="flex-1 px-4 py-3 border border-stone-300 rounded-xl text-sm resize-none focus:outline-none focus:ring-2 focus:ring-stone-500 disabled:opacity-50"
        placeholder="输入你的问题或观点... (Enter 发送, Shift+Enter 换行)"
      />
      <button
        @click="handleSend"
        :disabled="disabled || !input.trim()"
        class="px-5 py-3 bg-stone-800 text-white rounded-xl text-sm font-medium hover:bg-stone-700 transition-colors disabled:opacity-50"
      >
        发送
      </button>
    </div>
  </div>
</template>
