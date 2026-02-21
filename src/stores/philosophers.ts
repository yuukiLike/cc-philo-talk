import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { presetPhilosophers } from "@/modules/philosophers/data/presets";

export interface Philosopher {
  id: string;
  name: string;
  era: string;
  coreIdeas: string[];
  context: string;
  createdAt: string;
  updatedAt: string;
}

export const usePhilosophersStore = defineStore("philosophers", () => {
  const philosophers = ref<Philosopher[]>([]);
  const loading = ref(false);

  async function loadPhilosophers() {
    loading.value = true;
    try {
      const list = await invoke<Philosopher[]>("list_philosophers");
      if (list.length === 0) {
        // First launch: save preset philosophers
        for (const p of presetPhilosophers) {
          await invoke("save_philosopher", { philosopher: p });
        }
        philosophers.value = presetPhilosophers;
      } else {
        philosophers.value = list;
      }
    } catch (e) {
      console.error("Failed to load philosophers:", e);
    } finally {
      loading.value = false;
    }
  }

  async function savePhilosopher(philosopher: Philosopher) {
    await invoke("save_philosopher", { philosopher });
    const idx = philosophers.value.findIndex((p) => p.id === philosopher.id);
    if (idx >= 0) {
      philosophers.value[idx] = philosopher;
    } else {
      philosophers.value.push(philosopher);
    }
  }

  async function deletePhilosopher(id: string) {
    await invoke("delete_philosopher", { id });
    philosophers.value = philosophers.value.filter((p) => p.id !== id);
  }

  function getById(id: string): Philosopher | undefined {
    return philosophers.value.find((p) => p.id === id);
  }

  return {
    philosophers,
    loading,
    loadPhilosophers,
    savePhilosopher,
    deletePhilosopher,
    getById,
  };
});
