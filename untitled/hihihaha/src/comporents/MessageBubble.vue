<script setup lang="ts">
import type { Message } from "../types/message.ts";
import { computed, onMounted, onUnmounted, ref } from "vue";

const REACTION_EMOJIS = ["😱", "😂", "❤️", "🔥", "👍", "🎉", "🤔", "💪", "👋", "😍"];

const props = defineProps<{
  message: Message;
  isOwn: boolean;
}>();

const emit = defineEmits<{
  react: [messageId: number, emoji: string];
}>();

const isPickerOpen = ref(false);
const rootEl = ref<HTMLElement | null>(null);

const reactionGroups = computed(() => props.message.reactions ?? []);

function togglePicker() {
  isPickerOpen.value = !isPickerOpen.value;
}

function pickEmoji(emoji: string) {
  emit("react", props.message.id, emoji);
  isPickerOpen.value = false;
}

function onReactionClick(emoji: string) {
  emit("react", props.message.id, emoji);
}

function onDocumentClick(event: MouseEvent) {
  if (!isPickerOpen.value || !rootEl.value) return;
  if (!rootEl.value.contains(event.target as Node)) {
    isPickerOpen.value = false;
  }
}

onMounted(() => {
  document.addEventListener("click", onDocumentClick);
});

onUnmounted(() => {
  document.removeEventListener("click", onDocumentClick);
});
</script>

<template>
  <article
      ref="rootEl"
      class="message"
      :class="{
        'message--own': isOwn,
        'message--other': !isOwn,
      }"
  >
    <p>
      {{ message.body }}
    </p>

    <div v-if="reactionGroups.length > 0" class="reactions">
      <button
          v-for="group in reactionGroups"
          :key="group.emoji"
          type="button"
          class="reaction-chip"
          :class="{ 'reaction-chip--mine': group.reactedByMe }"
          :title="`${group.count}`"
          @click="onReactionClick(group.emoji)"
      >
        <span>{{ group.emoji }}</span>
        <span class="reaction-count">{{ group.count }}</span>
      </button>
    </div>

    <div class="message-actions">
      <button
          type="button"
          class="react-btn"
          title="Реакция"
          @click.stop="togglePicker"
      >
        😊
      </button>

      <div v-if="isPickerOpen" class="reaction-picker" @click.stop>
        <button
            v-for="emoji in REACTION_EMOJIS"
            :key="emoji"
            type="button"
            class="emoji-btn"
            @click="pickEmoji(emoji)"
        >
          {{ emoji }}
        </button>
      </div>
    </div>

    <footer>
      <span>{{ message.author }}</span>
      <span>{{ message.created_at }}</span>
    </footer>
  </article>
</template>

<style scoped>
.message {
  position: relative;
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
}

.message--own {
  align-self: flex-end;
  background: #17191f;
}

.message--other {
  align-self: flex-start;
  background: #343842;
}

.message p {
  margin: 0;
  line-height: 1.45;
  overflow-wrap: anywhere;
}

.message footer {
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 6px;
  color: #b5bbc7;
  font-size: 10px;
}

.reactions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-top: 8px;
}

.reaction-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 7px;
  border: 1px solid #3a3f4d;
  border-radius: 999px;
  background: #1e2026;
  color: #f2f3f5;
  font: inherit;
  font-size: 13px;
  cursor: pointer;
}

.reaction-chip--mine {
  border-color: #4a6cf7;
  background: #243056;
}

.reaction-count {
  font-size: 11px;
  color: #b5bbc7;
}

.message-actions {
  position: relative;
  margin-top: 6px;
}

.react-btn {
  padding: 2px 6px;
  border: none;
  border-radius: 6px;
  background: transparent;
  cursor: pointer;
  font-size: 14px;
  opacity: 0.55;
  transition: opacity 0.15s, background 0.15s;
}

.message:hover .react-btn,
.react-btn:focus-visible {
  opacity: 1;
}

.react-btn:hover {
  background: #292c34;
}

.reaction-picker {
  position: absolute;
  bottom: calc(100% + 6px);
  left: 0;
  z-index: 20;
  display: flex;
  flex-wrap: wrap;
  gap: 2px;
  max-width: 220px;
  padding: 8px;
  border: 1px solid #292c34;
  border-radius: 10px;
  background: #1e2026;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
}

.message--own .reaction-picker {
  left: auto;
  right: 0;
}

.emoji-btn {
  padding: 4px 6px;
  border: none;
  border-radius: 4px;
  background: transparent;
  cursor: pointer;
  font-size: 18px;
}

.emoji-btn:hover {
  background: #292c34;
}
</style>
