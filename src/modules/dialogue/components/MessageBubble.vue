<script setup lang="ts">
import type { Message } from "@/stores/dialogue";

defineProps<{
  message: Message;
  philosopherName: string;
}>();

defineEmits<{
  saveInsight: [];
}>();
</script>

<template>
  <div
    class="flex"
    :class="message.role === 'user' ? 'justify-end' : 'justify-start'"
  >
    <div
      class="max-w-[75%] rounded-2xl px-4 py-3 relative group"
      :class="
        message.role === 'user'
          ? 'bg-stone-800 text-white'
          : 'bg-white border border-stone-200 text-stone-800'
      "
    >
      <p v-if="message.role === 'philosopher'" class="text-xs font-medium text-stone-400 mb-1">
        {{ philosopherName }}
      </p>
      <p class="text-sm whitespace-pre-wrap leading-relaxed">{{ message.content }}</p>
      <p
        class="text-xs mt-1.5 opacity-60"
        :class="message.role === 'user' ? 'text-stone-300' : 'text-stone-400'"
      >
        {{ new Date(message.timestamp).toLocaleTimeString("zh-CN", { hour: "2-digit", minute: "2-digit" }) }}
      </p>

      <!-- Save as insight button -->
      <button
        v-if="message.role === 'philosopher' && !message.savedAsInsight"
        @click="$emit('saveInsight')"
        class="absolute -right-2 top-2 opacity-0 group-hover:opacity-100 bg-stone-800 text-white text-xs px-2 py-1 rounded-lg transition-opacity"
        title="收藏为感悟"
      >
        收藏
      </button>
      <span
        v-if="message.savedAsInsight"
        class="absolute -right-2 top-2 text-xs text-stone-400"
      >
        已收藏
      </span>
    </div>
  </div>
</template>
