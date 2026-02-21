<script setup lang="ts">
import type { Insight } from "@/stores/me";

defineProps<{
  insight: Insight;
}>();

defineEmits<{
  delete: [];
}>();
</script>

<template>
  <div class="bg-white border border-stone-200 rounded-xl p-4 group">
    <p class="text-sm text-stone-700 whitespace-pre-wrap leading-relaxed">
      {{ insight.content }}
    </p>
    <div class="flex items-center justify-between mt-3">
      <div class="flex items-center gap-2 text-xs text-stone-400">
        <span>{{ new Date(insight.createdAt).toLocaleDateString("zh-CN") }}</span>
        <span
          class="px-1.5 py-0.5 rounded-full"
          :class="insight.source === 'dialogue' ? 'bg-blue-50 text-blue-500' : 'bg-stone-100 text-stone-500'"
        >
          {{ insight.source === "dialogue" ? "对话" : "手记" }}
        </span>
      </div>
      <button
        @click="$emit('delete')"
        class="text-xs text-stone-400 hover:text-red-500 opacity-0 group-hover:opacity-100 transition-opacity"
      >
        删除
      </button>
    </div>
  </div>
</template>
