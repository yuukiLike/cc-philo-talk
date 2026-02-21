import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { usePhilosophersStore } from "./philosophers";
import { useMeStore } from "./me";

export interface Message {
  id: string;
  role: "user" | "philosopher";
  content: string;
  savedAsInsight: boolean;
  timestamp: string;
}

export interface Dialogue {
  id: string;
  philosopherId: string;
  messages: Message[];
  createdAt: string;
  updatedAt: string;
}

export interface DialogueSummary {
  id: string;
  philosopherId: string;
  messageCount: number;
  lastMessage: string | null;
  createdAt: string;
  updatedAt: string;
}

export const useDialogueStore = defineStore("dialogue", () => {
  const currentDialogue = ref<Dialogue | null>(null);
  const dialogueList = ref<DialogueSummary[]>([]);
  const sending = ref(false);
  const loading = ref(false);

  function buildSystemPrompt(philosopherId: string, includeMyContext: boolean): string {
    const philosophersStore = usePhilosophersStore();
    const philosopher = philosophersStore.getById(philosopherId);
    if (!philosopher) return "";

    let prompt = `你是${philosopher.name}（${philosopher.era}）。\n`;
    prompt += `以下是关于你的思想和立场的背景信息：\n\n${philosopher.context}\n\n`;

    if (includeMyContext) {
      const meStore = useMeStore();
      const myContext = meStore.myPhilosophy.context || meStore.myPhilosophy.manifesto;
      if (myContext) {
        prompt += `你的对话者有以下哲学立场：\n${myContext}\n`;
        prompt += `请在回应时考虑对方的立场，进行有深度的哲学对话。\n\n`;
      }
    }

    prompt += `请以${philosopher.name}的身份、语气和思维方式回应。不要跳出角色。用中文对话。`;
    return prompt;
  }

  function startNewDialogue(philosopherId: string) {
    currentDialogue.value = {
      id: crypto.randomUUID(),
      philosopherId,
      messages: [],
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
  }

  async function sendMessage(content: string, includeMyContext = true) {
    if (!currentDialogue.value) return;

    const userMessage: Message = {
      id: crypto.randomUUID(),
      role: "user",
      content,
      savedAsInsight: false,
      timestamp: new Date().toISOString(),
    };
    currentDialogue.value.messages.push(userMessage);

    sending.value = true;
    try {
      const systemPrompt = buildSystemPrompt(
        currentDialogue.value.philosopherId,
        includeMyContext,
      );

      const chatMessages = currentDialogue.value.messages.map((m) => ({
        role: m.role,
        content: m.content,
      }));

      const response = await invoke<string>("chat", {
        systemPrompt,
        messages: chatMessages,
      });

      const philosopherMessage: Message = {
        id: crypto.randomUUID(),
        role: "philosopher",
        content: response,
        savedAsInsight: false,
        timestamp: new Date().toISOString(),
      };
      currentDialogue.value.messages.push(philosopherMessage);
      currentDialogue.value.updatedAt = new Date().toISOString();

      // Auto-save dialogue
      await invoke("save_dialogue", { dialogue: currentDialogue.value });
    } catch (e) {
      // Remove the user message if AI call failed
      currentDialogue.value.messages.pop();
      throw e;
    } finally {
      sending.value = false;
    }
  }

  async function loadDialogueList(philosopherId?: string) {
    loading.value = true;
    try {
      dialogueList.value = await invoke<DialogueSummary[]>("list_dialogues", {
        philosopherId: philosopherId ?? null,
      });
    } catch (e) {
      console.error("Failed to load dialogues:", e);
    } finally {
      loading.value = false;
    }
  }

  async function loadDialogue(id: string) {
    const dialogue = await invoke<Dialogue | null>("get_dialogue", { id });
    if (dialogue) {
      currentDialogue.value = dialogue;
    }
  }

  return {
    currentDialogue,
    dialogueList,
    sending,
    loading,
    startNewDialogue,
    sendMessage,
    loadDialogueList,
    loadDialogue,
  };
});
