<script setup lang="ts">
import { nextTick, ref, useTemplateRef } from "vue";

const emit = defineEmits<{
  send: [body: string];
}>();

const draft = ref("");
const isEmojiPanelOpen = ref(false);
const cursorPos = ref(0);
const inputEl = useTemplateRef<HTMLInputElement>("draft-input");

function rememberCursor() {
  const el = inputEl.value;
  if (!el) return;
  cursorPos.value = el.selectionStart ?? draft.value.length;
}

function addEmoji(emoji: string) {
  const pos = cursorPos.value;
  const before = draft.value.slice(0, pos);
  const after = draft.value.slice(pos);

  draft.value = before + emoji + after;
  cursorPos.value = pos + emoji.length;

  nextTick(() => {
    const el = inputEl.value;
    if (!el) return;
    el.focus();
    el.setSelectionRange(cursorPos.value, cursorPos.value);
  });
}

function submitMessage() {
  const body = draft.value.trim();

  if (!body) return;

  emit("send", body);

  draft.value = "";
  cursorPos.value = 0;
}

function toggleEmojiPanel() {
  rememberCursor();
  isEmojiPanelOpen.value = !isEmojiPanelOpen.value;
}
</script>

<template>
  <form class="composer" @submit.prevent="submitMessage()">
    <div class="input-group">
      <input
          ref="draft-input"
          v-model="draft"
          type="text"
          placeholder="Ну пиши уже че нить"
          autocapitalize="off"
          @click="rememberCursor"
          @keyup="rememberCursor"
          @select="rememberCursor"
          @blur="rememberCursor"
      />
      <button type="button" class="emoji-toggle-btn" @mousedown.prevent @click="toggleEmojiPanel">
        😊
      </button>
      <button type="submit">Отправить</button>
    </div>

    <div v-if="isEmojiPanelOpen" class="emoji-panel">
      <div class="emoji-bar">
        <button type="button" @mousedown.prevent @click="addEmoji('😱')" class="emoji-btn">😱</button>
        <button type="button" @mousedown.prevent @click="addEmoji('😂')" class="emoji-btn">😂</button>
        <button type="button" @mousedown.prevent @click="addEmoji('❤️')" class="emoji-btn">❤️</button>
        <button type="button" @mousedown.prevent @click="addEmoji('🔥')" class="emoji-btn">🔥</button>
        <button type="button" @mousedown.prevent @click="addEmoji('👍')" class="emoji-btn">👍</button>
        <button type="button" @mousedown.prevent @click="addEmoji('🎉')" class="emoji-btn">🎉</button>
        <button type="button" @mousedown.prevent @click="addEmoji('🤔')" class="emoji-btn">🤔</button>
        <button type="button" @mousedown.prevent @click="addEmoji('💪')" class="emoji-btn">💪</button>
        <button type="button" @mousedown.prevent @click="addEmoji('👋')" class="emoji-btn">👋</button>
        <button type="button" @mousedown.prevent @click="addEmoji('😍')" class="emoji-btn">😍</button>
      </div>
    </div>
  </form>
</template>

<style scoped>
.composer {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px 20px;
  border-top: 1px solid #202332;
  flex-shrink: 0;
  background: #111318;
  position: relative;
}

.input-group {
  display: flex;
  gap: 10px;
}

.composer input {
  flex: 1;
  min-width: 0;
  padding: 11px 13px;
  border: 1px solid #292c34;
  border-radius: 7px;
  outline: none;
  color: #f2f3f5;
  background: #1e2026;
  font: inherit;
}

.composer button {
  padding: 0 24px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: white;
  background: #4a6cf7;
  font: inherit;
}

.emoji-toggle-btn {
  padding: 0 14px;
  flex-shrink: 0;
  background: #292c34;
}

.emoji-panel {
  position: absolute;
  bottom: calc(100% + 8px);
  right: 20px;
  left: auto;
  background: #1e2026;
  border: 1px solid #292c34;
  border-radius: 10px;
  padding: 12px 14px;
  z-index: 10;
}

.emoji-bar {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.emoji-btn {
  padding: 4px 8px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  background: transparent;
  font-size: 18px;
  transition: background 0.15s;
}

.emoji-btn:hover {
  background: #292c34;
}
</style>
