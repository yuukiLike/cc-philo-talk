<script setup lang="ts">
import { ref } from "vue";
import type { Philosopher } from "@/stores/philosophers";

const props = defineProps<{
  philosopher: Philosopher;
  isNew: boolean;
}>();

const emit = defineEmits<{
  save: [philosopher: Philosopher];
  cancel: [];
}>();

const form = ref<Philosopher>({ ...props.philosopher });
const newIdea = ref("");

function addIdea() {
  const idea = newIdea.value.trim();
  if (idea && !form.value.coreIdeas.includes(idea)) {
    form.value.coreIdeas.push(idea);
    newIdea.value = "";
  }
}

function removeIdea(index: number) {
  form.value.coreIdeas.splice(index, 1);
}

function save() {
  if (!form.value.name.trim()) return;
  emit("save", form.value);
}
</script>

<template>
  <div class="bg-white rounded-xl shadow-xl w-full max-w-2xl max-h-[85vh] overflow-y-auto p-6">
    <h2 class="text-xl font-bold text-stone-900 mb-4">
      {{ isNew ? "添加哲学家" : `编辑 ${philosopher.name}` }}
    </h2>

    <div class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-stone-700 mb-1">姓名</label>
          <input
            v-model="form.name"
            class="w-full px-3 py-2 border border-stone-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-stone-500"
            placeholder="例：柏拉图"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-stone-700 mb-1">年代</label>
          <input
            v-model="form.era"
            class="w-full px-3 py-2 border border-stone-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-stone-500"
            placeholder="例：前427—前347"
          />
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-stone-700 mb-1">核心思想</label>
        <div class="flex flex-wrap gap-1.5 mb-2">
          <span
            v-for="(idea, i) in form.coreIdeas"
            :key="i"
            class="px-2 py-0.5 bg-stone-100 text-stone-600 text-xs rounded-full flex items-center gap-1"
          >
            {{ idea }}
            <button @click="removeIdea(i)" class="text-stone-400 hover:text-stone-600">&times;</button>
          </span>
        </div>
        <div class="flex gap-2">
          <input
            v-model="newIdea"
            @keyup.enter="addIdea"
            class="flex-1 px-3 py-2 border border-stone-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-stone-500"
            placeholder="输入一个核心思想，按回车添加"
          />
          <button
            @click="addIdea"
            class="px-3 py-2 bg-stone-100 text-stone-700 rounded-lg text-sm hover:bg-stone-200"
          >
            添加
          </button>
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-stone-700 mb-1">
          思想上下文
          <span class="text-stone-400 font-normal">（AI 对话时的背景知识，越详细越好）</span>
        </label>
        <textarea
          v-model="form.context"
          rows="12"
          class="w-full px-3 py-2 border border-stone-300 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-stone-500 font-mono"
          placeholder="详细描述这位哲学家的核心思想、哲学立场、论证方法、对话风格..."
        />
      </div>
    </div>

    <div class="flex justify-end gap-3 mt-6">
      <button
        @click="emit('cancel')"
        class="px-4 py-2 text-stone-600 text-sm hover:bg-stone-100 rounded-lg transition-colors"
      >
        取消
      </button>
      <button
        @click="save"
        :disabled="!form.name.trim()"
        class="px-4 py-2 bg-stone-800 text-white rounded-lg text-sm hover:bg-stone-700 transition-colors disabled:opacity-50"
      >
        保存
      </button>
    </div>
  </div>
</template>
