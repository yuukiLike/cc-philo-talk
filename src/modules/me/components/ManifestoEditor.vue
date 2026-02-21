<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  manifesto: string;
}>();

const emit = defineEmits<{
  save: [manifesto: string];
  cancel: [];
}>();

const draft = ref(props.manifesto);
</script>

<template>
  <div class="bg-white border border-stone-200 rounded-xl p-4">
    <textarea
      v-model="draft"
      rows="8"
      class="w-full px-3 py-2 border border-stone-300 rounded-lg text-sm resize-none focus:outline-none focus:ring-2 focus:ring-stone-500"
      placeholder="我的哲学宣言——我相信什么，我的核心立场是什么..."
      autofocus
    />
    <div class="flex justify-end gap-2 mt-3">
      <button
        @click="emit('cancel')"
        class="px-3 py-1.5 text-sm text-stone-600 hover:bg-stone-100 rounded-lg transition-colors"
      >
        取消
      </button>
      <button
        @click="emit('save', draft)"
        class="px-3 py-1.5 bg-stone-800 text-white rounded-lg text-sm hover:bg-stone-700 transition-colors"
      >
        保存
      </button>
    </div>
  </div>
</template>
