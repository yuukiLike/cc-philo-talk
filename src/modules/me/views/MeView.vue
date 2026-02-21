<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useMeStore } from "@/stores/me";
import InsightCard from "../components/InsightCard.vue";
import InsightEditor from "../components/InsightEditor.vue";
import ManifestoEditor from "../components/ManifestoEditor.vue";

const meStore = useMeStore();
const showInsightEditor = ref(false);
const editingManifesto = ref(false);
const editingContext = ref(false);
const contextDraft = ref("");

onMounted(() => {
  meStore.loadMyPhilosophy();
});

function startEditContext() {
  contextDraft.value = meStore.myPhilosophy.context;
  editingContext.value = true;
}

async function saveContext() {
  meStore.myPhilosophy.context = contextDraft.value;
  await meStore.saveMyPhilosophy();
  editingContext.value = false;
}

async function handleAddInsight(content: string) {
  await meStore.addInsight(content, "manual");
  showInsightEditor.value = false;
}

async function handleSaveManifesto(manifesto: string) {
  meStore.myPhilosophy.manifesto = manifesto;
  await meStore.saveMyPhilosophy();
  editingManifesto.value = false;
}

async function deleteInsight(id: string) {
  meStore.myPhilosophy.insights = meStore.myPhilosophy.insights.filter(
    (i) => i.id !== id,
  );
  await meStore.saveMyPhilosophy();
}
</script>

<template>
  <div class="p-6 max-w-4xl mx-auto">
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-stone-900">{{ meStore.myPhilosophy.name }}</h1>
      <p class="text-stone-500 text-sm mt-1">你的哲学，从你独特的生命经验中诞生</p>
    </div>

    <!-- Manifesto Section -->
    <section class="mb-8">
      <div class="flex items-center justify-between mb-3">
        <h2 class="text-lg font-semibold text-stone-800">哲学宣言</h2>
        <button
          @click="editingManifesto = !editingManifesto"
          class="text-sm text-stone-500 hover:text-stone-700"
        >
          {{ editingManifesto ? "取消" : "编辑" }}
        </button>
      </div>

      <ManifestoEditor
        v-if="editingManifesto"
        :manifesto="meStore.myPhilosophy.manifesto"
        @save="handleSaveManifesto"
        @cancel="editingManifesto = false"
      />

      <div
        v-else-if="meStore.myPhilosophy.manifesto"
        class="bg-white border border-stone-200 rounded-xl p-5 whitespace-pre-wrap text-sm text-stone-700 leading-relaxed"
      >
        {{ meStore.myPhilosophy.manifesto }}
      </div>

      <div
        v-else
        class="bg-stone-50 border border-dashed border-stone-300 rounded-xl p-5 text-center text-stone-400 text-sm cursor-pointer hover:bg-stone-100 transition-colors"
        @click="editingManifesto = true"
      >
        写下你的哲学宣言——你的核心立场和信念
      </div>
    </section>

    <!-- Context Section -->
    <section class="mb-8">
      <div class="flex items-center justify-between mb-3">
        <div>
          <h2 class="text-lg font-semibold text-stone-800">思想上下文</h2>
          <p class="text-stone-400 text-xs mt-0.5">对话时 AI 会参考这段内容来理解你的立场</p>
        </div>
        <button
          @click="editingContext ? saveContext() : startEditContext()"
          class="text-sm text-stone-500 hover:text-stone-700"
        >
          {{ editingContext ? "保存" : "编辑" }}
        </button>
      </div>

      <textarea
        v-if="editingContext"
        v-model="contextDraft"
        rows="8"
        class="w-full px-4 py-3 border border-stone-300 rounded-xl text-sm font-mono focus:outline-none focus:ring-2 focus:ring-stone-500"
        placeholder="描述你的哲学立场、思考方式、关注的问题..."
      />

      <div
        v-else-if="meStore.myPhilosophy.context"
        class="bg-white border border-stone-200 rounded-xl p-5 whitespace-pre-wrap text-sm text-stone-600 font-mono leading-relaxed"
      >
        {{ meStore.myPhilosophy.context }}
      </div>

      <div
        v-else
        class="bg-stone-50 border border-dashed border-stone-300 rounded-xl p-5 text-center text-stone-400 text-sm cursor-pointer hover:bg-stone-100 transition-colors"
        @click="startEditContext"
      >
        添加你的思想上下文，让哲学家在对话中更了解你
      </div>
    </section>

    <!-- Insights Section -->
    <section>
      <div class="flex items-center justify-between mb-3">
        <h2 class="text-lg font-semibold text-stone-800">
          我的感悟
          <span class="text-stone-400 font-normal text-sm ml-2">
            {{ meStore.myPhilosophy.insights.length }}
          </span>
        </h2>
        <button
          @click="showInsightEditor = !showInsightEditor"
          class="px-3 py-1.5 bg-stone-800 text-white rounded-lg text-sm hover:bg-stone-700 transition-colors"
        >
          + 添加感悟
        </button>
      </div>

      <InsightEditor
        v-if="showInsightEditor"
        @save="handleAddInsight"
        @cancel="showInsightEditor = false"
      />

      <div v-if="meStore.myPhilosophy.insights.length === 0 && !showInsightEditor" class="text-center text-stone-400 text-sm py-8">
        还没有感悟。和哲学家对话时可以收藏，也可以手动添加。
      </div>

      <div class="space-y-3">
        <InsightCard
          v-for="insight in [...meStore.myPhilosophy.insights].reverse()"
          :key="insight.id"
          :insight="insight"
          @delete="deleteInsight(insight.id)"
        />
      </div>
    </section>
  </div>
</template>
