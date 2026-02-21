<script setup lang="ts">
import type { DialogueSummary } from "@/stores/dialogue";
import type { Philosopher } from "@/stores/philosophers";

defineProps<{
  dialogues: DialogueSummary[];
  philosophers: Philosopher[];
}>();

const emit = defineEmits<{
  select: [id: string];
}>();

function getPhilosopherName(id: string, philosophers: Philosopher[]): string {
  return philosophers.find((p) => p.id === id)?.name || id;
}
</script>

<template>
  <div class="border-b border-stone-200 bg-stone-50 px-6 py-3 max-h-60 overflow-y-auto">
    <div v-if="dialogues.length === 0" class="text-stone-400 text-sm text-center py-4">
      暂无历史对话
    </div>
    <div
      v-for="d in dialogues"
      :key="d.id"
      @click="emit('select', d.id)"
      class="py-2 px-3 hover:bg-stone-100 rounded-lg cursor-pointer transition-colors"
    >
      <div class="flex items-center justify-between">
        <span class="text-sm font-medium text-stone-700">
          与{{ getPhilosopherName(d.philosopherId, philosophers) }}的对话
        </span>
        <span class="text-xs text-stone-400">
          {{ new Date(d.updatedAt).toLocaleDateString("zh-CN") }}
        </span>
      </div>
      <p v-if="d.lastMessage" class="text-xs text-stone-500 mt-0.5 truncate">
        {{ d.lastMessage }}
      </p>
      <span class="text-xs text-stone-400">{{ d.messageCount }} 条消息</span>
    </div>
  </div>
</template>
