<script setup lang="ts">
import type { Message } from "../types/message.ts";
import { computed, onMounted, onUnmounted, ref } from "vue";
import EmojiPanel from "../emoji/EmojiPanel.vue";

const props = defineProps<{
  message: Message;
  isOwn: boolean;
  avatar: string;
}>();

const emit = defineEmits<{
  react: [messageId: number, emoji: string];
}>();

const isPickerOpen = ref(false);
const rootEl = ref<HTMLElement | null>(null);

const reactionGroups = computed(() => props.message.reactions ?? []);
const hasBody = computed(() => Boolean(props.message.body?.trim()));
const hasSticker = computed(() => Boolean(props.message.sticker));

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
    <img
        v-if="avatar"
        :src="avatar"
        :alt="message.author"
        class="message-avatar"
    />

    <div class="message-body">
      <p v-if="hasBody">
        {{ message.body }}
      </p>

      <img
          v-if="hasSticker"
          :src="message.sticker!"
          alt="Стикер"
          class="message-sticker"
      />

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

        <EmojiPanel
            v-if="isPickerOpen"
            class="reaction-emoji-panel"
            :show-stickers="false"
            @select="pickEmoji"
        />
      </div>

      <footer>
        <span>{{ message.author }}</span>
        <span>{{ message.created_at }}</span>
      </footer>
    </div>
  </article>
</template>

<style scoped>
.message {
  position: relative;
  display: flex;
  align-items: flex-end;
  gap: 8px;
  max-width: 70%;
  margin: 0;
}

.message-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
  flex-shrink: 0;
}

.message-body {
  min-width: 0;
  padding: 10px 12px;
  border-radius: 10px;
}

.message--own {
  align-self: flex-end;
  flex-direction: row-reverse;
}

.message--own .message-body {
  background: #17191f;
}

.message--other {
  align-self: flex-start;
}

.message--other .message-body {
  background: #343842;
}

.message-body p {
  margin: 0;
  line-height: 1.45;
  overflow-wrap: anywhere;
}

.message-sticker {
  display: block;
  max-width: 160px;
  max-height: 160px;
  margin-top: 4px;
  object-fit: contain;
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

.reaction-emoji-panel {
  position: absolute;
  bottom: calc(100% + 6px);
  left: 0;
  z-index: 20;
}

.message--own .reaction-emoji-panel {
  left: auto;
  right: 0;
}
</style>
