<script setup lang="ts">
import { useRoute } from "vue-router";

const route = useRoute();

const modules = [
  {
    id: "philosophers",
    name: "哲学家",
    path: "/philosophers",
    icon: "哲",
  },
  {
    id: "dialogue",
    name: "对话",
    path: "/dialogue",
    icon: "话",
  },
  {
    id: "me",
    name: "我",
    path: "/me",
    icon: "我",
  },
];
</script>

<template>
  <aside
    class="w-14 flex flex-col items-center py-4 gap-2 border-r border-stone-200 bg-stone-100/80"
  >
    <RouterLink
      v-for="mod in modules"
      :key="mod.id"
      :to="mod.path"
      :title="mod.name"
      class="w-10 h-10 flex items-center justify-center rounded-lg text-sm font-medium transition-colors"
      :class="
        route.path.startsWith(mod.path)
          ? 'bg-stone-800 text-white'
          : 'text-stone-500 hover:bg-stone-200 hover:text-stone-900'
      "
    >
      {{ mod.icon }}
    </RouterLink>
  </aside>
</template>
