<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import type { Philosopher } from "@/stores/philosophers";

const props = defineProps<{
  philosopher: Philosopher;
  size: "lg" | "md" | "sm";
}>();

defineEmits<{
  edit: [];
}>();

const router = useRouter();

function startDialogue() {
  router.push(`/dialogue/${props.philosopher.id}`);
}

// Large cards show more content
const contextPreview = computed(() => {
  const len = props.size === "lg" ? 300 : props.size === "md" ? 160 : 80;
  const ctx = props.philosopher.context;
  return ctx.length > len ? ctx.slice(0, len) + "..." : ctx;
});

const ideasToShow = computed(() => {
  const count = props.size === "lg" ? 6 : props.size === "md" ? 4 : 3;
  return props.philosopher.coreIdeas.slice(0, count);
});

// Color palette — each philosopher gets a subtle accent
const accentMap: Record<string, string> = {
  socrates: "from-amber-50 to-orange-50 border-amber-200",
  kant: "from-blue-50 to-indigo-50 border-blue-200",
  hegel: "from-purple-50 to-violet-50 border-purple-200",
  kierkegaard: "from-rose-50 to-pink-50 border-rose-200",
  marx: "from-red-50 to-orange-50 border-red-200",
  nietzsche: "from-yellow-50 to-amber-50 border-yellow-200",
  heidegger: "from-emerald-50 to-teal-50 border-emerald-200",
  foucault: "from-slate-50 to-gray-50 border-slate-200",
  derrida: "from-cyan-50 to-sky-50 border-cyan-200",
};

const accent = computed(() => accentMap[props.philosopher.id] || "from-stone-50 to-stone-100 border-stone-200");

const tagColor: Record<string, string> = {
  socrates: "bg-amber-100 text-amber-700",
  kant: "bg-blue-100 text-blue-700",
  hegel: "bg-purple-100 text-purple-700",
  kierkegaard: "bg-rose-100 text-rose-700",
  marx: "bg-red-100 text-red-700",
  nietzsche: "bg-yellow-100 text-yellow-700",
  heidegger: "bg-emerald-100 text-emerald-700",
  foucault: "bg-slate-100 text-slate-700",
  derrida: "bg-cyan-100 text-cyan-700",
};

const tag = computed(() => tagColor[props.philosopher.id] || "bg-stone-100 text-stone-600");
</script>

<template>
  <div
    class="group relative rounded-2xl border bg-gradient-to-br p-5 flex flex-col transition-all duration-300 hover:shadow-lg hover:-translate-y-0.5 overflow-hidden"
    :class="accent"
  >
    <!-- Header -->
    <div class="flex items-start justify-between mb-3">
      <div>
        <h3
          class="font-bold text-stone-900"
          :class="size === 'lg' ? 'text-2xl' : size === 'md' ? 'text-lg' : 'text-base'"
        >
          {{ philosopher.name }}
        </h3>
        <p class="text-stone-400 text-sm mt-0.5">{{ philosopher.era }}</p>
      </div>
      <button
        @click="$emit('edit')"
        class="text-stone-400 hover:text-stone-600 text-xs opacity-0 group-hover:opacity-100 transition-opacity"
        title="编辑"
      >
        编辑
      </button>
    </div>

    <!-- Core Ideas -->
    <div class="flex flex-wrap gap-1.5 mb-3">
      <span
        v-for="idea in ideasToShow"
        :key="idea"
        class="px-2 py-0.5 text-xs rounded-full font-medium"
        :class="tag"
      >
        {{ idea }}
      </span>
    </div>

    <!-- Context Preview (medium & large only) -->
    <p
      v-if="size !== 'sm'"
      class="text-stone-500 text-sm leading-relaxed flex-1"
      :class="size === 'lg' ? 'line-clamp-6' : 'line-clamp-3'"
    >
      {{ contextPreview }}
    </p>

    <!-- Spacer for small cards -->
    <div v-else class="flex-1" />

    <!-- Action -->
    <button
      @click="startDialogue"
      class="mt-4 w-full py-2.5 rounded-xl text-sm font-medium transition-all duration-200"
      :class="
        size === 'lg'
          ? 'bg-stone-800 text-white hover:bg-stone-700'
          : 'bg-white/60 text-stone-700 hover:bg-white border border-stone-200/60'
      "
    >
      {{ size === "lg" ? "与 " + philosopher.name + " 对话" : "对话" }}
    </button>

    <!-- Decorative element for large cards -->
    <div
      v-if="size === 'lg'"
      class="absolute -right-8 -bottom-8 w-32 h-32 rounded-full bg-white/20 blur-2xl"
    />
  </div>
</template>
