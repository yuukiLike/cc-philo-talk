<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits<{
  save: [content: string];
  cancel: [];
}>();

const content = ref("");

function save() {
  const text = content.value.trim();
  if (!text) return;
  emit("save", text);
  content.value = "";
}
</script>

<template>
  <div class="bg-white border border-stone-200 rounded-xl p-4 mb-4">
    <textarea
      v-model="content"
      rows="4"
      class="w-full px-3 py-2 border border-stone-300 rounded-lg text-sm resize-none focus:outline-none focus:ring-2 focus:ring-stone-500"
      placeholder="记录你的感悟、观点、思考..."
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
        @click="save"
        :disabled="!content.trim()"
        class="px-3 py-1.5 bg-stone-800 text-white rounded-lg text-sm hover:bg-stone-700 transition-colors disabled:opacity-50"
      >
        保存
      </button>
    </div>
  </div>
</template>
