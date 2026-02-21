<script setup lang="ts">
import { onMounted, ref, nextTick, computed } from "vue";
import { useRoute } from "vue-router";
import { useDialogueStore } from "@/stores/dialogue";
import { usePhilosophersStore } from "@/stores/philosophers";
import { useMeStore } from "@/stores/me";
import MessageBubble from "../components/MessageBubble.vue";
import MessageInput from "../components/MessageInput.vue";
import DialogueList from "../components/DialogueList.vue";

const route = useRoute();
const dialogueStore = useDialogueStore();
const philosophersStore = usePhilosophersStore();
const meStore = useMeStore();

const messagesContainer = ref<HTMLElement | null>(null);
const includeMyContext = ref(true);
const error = ref<string | null>(null);
const showHistory = ref(false);

const philosopherId = computed(() => route.params.philosopherId as string | undefined);
const philosopher = computed(() =>
  philosopherId.value ? philosophersStore.getById(philosopherId.value) : null,
);

onMounted(async () => {
  await philosophersStore.loadPhilosophers();
  await meStore.loadMyPhilosophy();

  if (philosopherId.value && !dialogueStore.currentDialogue) {
    dialogueStore.startNewDialogue(philosopherId.value);
  }
});

async function handleSend(content: string) {
  error.value = null;
  try {
    await dialogueStore.sendMessage(content, includeMyContext.value);
    await nextTick();
    scrollToBottom();
  } catch (e) {
    error.value = String(e);
  }
}

function scrollToBottom() {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
  }
}

async function saveAsInsight(content: string) {
  await meStore.addInsight(content, "dialogue", dialogueStore.currentDialogue?.id);
}

function startNewChat() {
  if (philosopherId.value) {
    dialogueStore.startNewDialogue(philosopherId.value);
  }
}

async function loadHistory() {
  showHistory.value = !showHistory.value;
  if (showHistory.value) {
    await dialogueStore.loadDialogueList(philosopherId.value);
  }
}

async function selectDialogue(id: string) {
  await dialogueStore.loadDialogue(id);
  showHistory.value = false;
}
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- No philosopher selected -->
    <div
      v-if="!philosopherId"
      class="flex-1 flex items-center justify-center text-stone-400"
    >
      <div class="text-center">
        <p class="text-lg mb-2">请先选择一位哲学家</p>
        <RouterLink
          to="/philosophers"
          class="text-stone-600 underline hover:text-stone-900"
        >
          前往哲学家列表
        </RouterLink>
      </div>
    </div>

    <!-- Dialogue interface -->
    <template v-else>
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-3 border-b border-stone-200 bg-white">
        <div class="flex items-center gap-3">
          <h2 class="font-bold text-stone-900">
            与{{ philosopher?.name || "..." }}对话
          </h2>
          <label class="flex items-center gap-1.5 text-xs text-stone-500">
            <input
              type="checkbox"
              v-model="includeMyContext"
              class="rounded"
            />
            注入"我"的立场
          </label>
        </div>
        <div class="flex gap-2">
          <button
            @click="loadHistory"
            class="px-3 py-1.5 text-sm text-stone-600 hover:bg-stone-100 rounded-lg transition-colors"
          >
            {{ showHistory ? "关闭历史" : "历史对话" }}
          </button>
          <button
            @click="startNewChat"
            class="px-3 py-1.5 text-sm bg-stone-800 text-white rounded-lg hover:bg-stone-700 transition-colors"
          >
            新对话
          </button>
        </div>
      </div>

      <!-- History panel -->
      <DialogueList
        v-if="showHistory"
        :dialogues="dialogueStore.dialogueList"
        :philosophers="philosophersStore.philosophers"
        @select="selectDialogue"
      />

      <!-- Messages -->
      <div
        ref="messagesContainer"
        class="flex-1 overflow-y-auto px-6 py-4 space-y-4"
      >
        <div
          v-if="!dialogueStore.currentDialogue?.messages.length"
          class="text-center text-stone-400 py-12"
        >
          <p class="text-lg mb-1">开始与{{ philosopher?.name }}的哲学对话</p>
          <p class="text-sm">提出你的问题或观点，{{ philosopher?.name }}会以其哲学立场回应你</p>
        </div>

        <MessageBubble
          v-for="msg in dialogueStore.currentDialogue?.messages"
          :key="msg.id"
          :message="msg"
          :philosopher-name="philosopher?.name || ''"
          @save-insight="saveAsInsight(msg.content)"
        />

        <div v-if="dialogueStore.sending" class="flex justify-start">
          <div class="bg-stone-100 rounded-2xl px-4 py-3 text-stone-500 text-sm">
            {{ philosopher?.name }}正在思考...
          </div>
        </div>
      </div>

      <!-- Error -->
      <div v-if="error" class="px-6 py-2 bg-red-50 text-red-600 text-sm">
        {{ error }}
      </div>

      <!-- Input -->
      <MessageInput
        :disabled="dialogueStore.sending"
        @send="handleSend"
      />
    </template>
  </div>
</template>
