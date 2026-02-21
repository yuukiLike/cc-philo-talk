<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { usePhilosophersStore } from "@/stores/philosophers";
import PhilosopherCard from "../components/PhilosopherCard.vue";
import PhilosopherEditor from "../components/PhilosopherEditor.vue";
import type { Philosopher } from "@/stores/philosophers";

const store = usePhilosophersStore();
const editingPhilosopher = ref<Philosopher | null>(null);
const showNewForm = ref(false);

// Bento layout: assign sizes based on philosopher significance/system size
const bentoSizeMap: Record<string, "lg" | "md" | "sm"> = {
  socrates: "lg",   // 哲学之父，最大展示
  kant: "md",       // 三大批判体系
  hegel: "md",      // 绝对精神体系
  nietzsche: "md",  // 需要空间展现张力
  kierkegaard: "sm",
  marx: "sm",
  heidegger: "sm",
  foucault: "sm",
  derrida: "sm",
};

function getBentoSize(id: string): "lg" | "md" | "sm" {
  return bentoSizeMap[id] || "sm";
}

function getBentoClass(id: string): string {
  const size = getBentoSize(id);
  switch (size) {
    case "lg":
      return "col-span-2 row-span-2";
    case "md":
      return "col-span-2";
    default:
      return "";
  }
}

// Sort: large first, then medium, then small — for better grid auto-flow
const sortedPhilosophers = computed(() => {
  const order = { lg: 0, md: 1, sm: 2 };
  return [...store.philosophers].sort((a, b) => {
    return order[getBentoSize(a.id)] - order[getBentoSize(b.id)];
  });
});

onMounted(() => {
  store.loadPhilosophers();
});

function openEditor(philosopher: Philosopher) {
  editingPhilosopher.value = { ...philosopher };
}

function closeEditor() {
  editingPhilosopher.value = null;
  showNewForm.value = false;
}

function startNew() {
  const now = new Date().toISOString();
  editingPhilosopher.value = {
    id: crypto.randomUUID(),
    name: "",
    era: "",
    coreIdeas: [],
    context: "",
    createdAt: now,
    updatedAt: now,
  };
  showNewForm.value = true;
}

async function handleSave(philosopher: Philosopher) {
  philosopher.updatedAt = new Date().toISOString();
  await store.savePhilosopher(philosopher);
  closeEditor();
}
</script>

<template>
  <div class="p-6 max-w-6xl mx-auto">
    <div class="flex items-center justify-between mb-8">
      <div>
        <h1 class="text-2xl font-bold text-stone-900">哲学家</h1>
        <p class="text-stone-500 text-sm mt-1">选择一位哲学家开始对话，或编辑他们的思想上下文</p>
      </div>
      <button
        @click="startNew"
        class="px-4 py-2 bg-stone-800 text-white rounded-lg text-sm hover:bg-stone-700 transition-colors"
      >
        + 添加哲学家
      </button>
    </div>

    <div v-if="store.loading" class="text-stone-500 text-center py-12">
      加载中...
    </div>

    <!-- Bento Grid -->
    <div
      v-else
      class="grid grid-cols-2 lg:grid-cols-4 gap-4 auto-rows-[minmax(180px,auto)]"
      style="grid-auto-flow: dense"
    >
      <PhilosopherCard
        v-for="p in sortedPhilosophers"
        :key="p.id"
        :philosopher="p"
        :size="getBentoSize(p.id)"
        :class="getBentoClass(p.id)"
        @edit="openEditor(p)"
      />
    </div>

    <!-- Editor Modal -->
    <div
      v-if="editingPhilosopher"
      class="fixed inset-0 bg-black/40 flex items-center justify-center z-50 p-4"
      @click.self="closeEditor"
    >
      <PhilosopherEditor
        :philosopher="editingPhilosopher"
        :is-new="showNewForm"
        @save="handleSave"
        @cancel="closeEditor"
      />
    </div>
  </div>
</template>
