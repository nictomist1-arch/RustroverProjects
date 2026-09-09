<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits<{
  send: [body: string];
}>();

const draft = ref("");
const isEmojiPanelOpen = ref(false);

function addEmoji(emoji: string) {
  draft.value += emoji;
}

function submitMessage() {
  const body = draft.value.trim();

  if (!body) return;

  emit("send", body);

  draft.value = "";
}

function toggleEmojiPanel() {
  isEmojiPanelOpen.value = !isEmojiPanelOpen.value;
}

</script>

<template>
  <form class="composer" @submit.prevent="submitMessage()">
    <div class="input-group">
      <input
          v-model="draft"
          type="text"
          placeholder="Ну пиши уже че нить"
          autocapitalize="off"
      />
      <button type="button" class="emoji-toggle-btn" @click="toggleEmojiPanel">
        😊
      </button>
      <button type="submit">Отправить</button>
    </div>

    <div v-if="isEmojiPanelOpen" class="emoji-panel">
      <div class="emoji-bar">
        <button type="button" @click="addEmoji('😱')" class="emoji-btn">😱</button>
        <button type="button" @click="addEmoji('😂')" class="emoji-btn">😂</button>
        <button type="button" @click="addEmoji('❤️')" class="emoji-btn">❤️</button>
        <button type="button" @click="addEmoji('🔥')" class="emoji-btn">🔥</button>
        <button type="button" @click="addEmoji('👍')" class="emoji-btn">👍</button>
        <button type="button" @click="addEmoji('🎉')" class="emoji-btn">🎉</button>
        <button type="button" @click="addEmoji('🤔')" class="emoji-btn">🤔</button>
        <button type="button" @click="addEmoji('💪')" class="emoji-btn">💪</button>
        <button type="button" @click="addEmoji('👋')" class="emoji-btn">👋</button>
        <button type="button" @click="addEmoji('😍')" class="emoji-btn">😍</button>
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

.emoji-panel {
  position: absolute;
  bottom: 80px;
  left: 1500px;
  right: 20px;
  background: #1e2026;
  border: 1px solid #292c34;
  border-radius: 10px;
  padding: 12px 14px;
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