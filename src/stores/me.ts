import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface Insight {
  id: string;
  content: string;
  source: "dialogue" | "manual";
  dialogueId?: string;
  createdAt: string;
}

export interface MyPhilosophy {
  name: string;
  manifesto: string;
  insights: Insight[];
  context: string;
  updatedAt: string;
}

export const useMeStore = defineStore("me", () => {
  const myPhilosophy = ref<MyPhilosophy>({
    name: "我",
    manifesto: "",
    insights: [],
    context: "",
    updatedAt: new Date().toISOString(),
  });
  const loading = ref(false);

  async function loadMyPhilosophy() {
    loading.value = true;
    try {
      myPhilosophy.value = await invoke<MyPhilosophy>("get_my_philosophy");
    } catch (e) {
      console.error("Failed to load my philosophy:", e);
    } finally {
      loading.value = false;
    }
  }

  async function saveMyPhilosophy() {
    myPhilosophy.value.updatedAt = new Date().toISOString();
    await invoke("save_my_philosophy", { data: myPhilosophy.value });
  }

  async function addInsight(content: string, source: "dialogue" | "manual", dialogueId?: string) {
    const insight: Insight = {
      id: crypto.randomUUID(),
      content,
      source,
      dialogueId,
      createdAt: new Date().toISOString(),
    };
    await invoke("add_insight", { insight });
    myPhilosophy.value.insights.push(insight);
  }

  return {
    myPhilosophy,
    loading,
    loadMyPhilosophy,
    saveMyPhilosophy,
    addInsight,
  };
});
