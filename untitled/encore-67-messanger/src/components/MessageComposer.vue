<script setup lang="ts">
import { nextTick, ref, useTemplateRef } from "vue";

import { open } from "@tauri-apps/plugin-dialog";

import { invoke } from "@tauri-apps/api/core";

import EmojiPanel from "../emoji/EmojiPanel.vue";

const emit = defineEmits<{
  send: [body: string];
  sendImage: [path: string];
  sendSticker: [src: string];
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
  draft.value = draft.value.slice(0, pos) + emoji + draft.value.slice(pos);
  cursorPos.value = pos + emoji.length;

  nextTick(() => {
    const el = inputEl.value;
    if (!el) return;
    el.focus();
    el.setSelectionRange(cursorPos.value, cursorPos.value);
  });
}

function sendSticker(src: string) {
  emit("sendSticker", src);
  isEmojiPanelOpen.value = false;
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

async function selectImage() {
  const file = await open({
    multiple: false,
    filters: [
      {
        name: "Image",
        extensions: ["png", "jpg", "jpeg", "webp", "gif"],
      },
    ],
  });

  if (!file) {
    return;
  }

  const savedPath = await invoke<string>("save_attachment", {
    source: file,
  });

  emit("sendImage", savedPath);
}
</script>

<template>
  <form class="composer" @submit.prevent="submitMessage">
    <div class="input-group">
      <button
        type="button"
        class="icon-btn"
        title="Прикрепить файл"
        @mousedown.prevent
        @click="selectImage"
      >
        📎
      </button>
      <input
        ref="draft-input"
        v-model="draft"
        type="text"
        placeholder="Ну пиши уже че нить"
        autocomplete="off"
        @click="rememberCursor"
        @keyup="rememberCursor"
        @select="rememberCursor"
        @blur="rememberCursor"
      />
      <button
        type="button"
        class="icon-btn"
        title="Эмодзи и стикеры"
        @mousedown.prevent
        @click="toggleEmojiPanel"
      >
        😊
      </button>
      <button type="submit">Отправить</button>
    </div>

    <EmojiPanel
      v-if="isEmojiPanelOpen"
      class="composer-emoji-panel"
      prevent-mouse-down
      show-stickers
      @select="addEmoji"
      @select-sticker="sendSticker"
    />
  </form>
</template>

<style scoped>
.composer {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 15px 20px;
  border-top: 1px solid #252830;
  background: #17191f;
  flex-shrink: 0;
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
  border: 1px solid #343842;
  border-radius: 7px;
  outline: none;
  color: #f2f3f5;
  background: #20232a;
  font: inherit;
}

.composer input:focus {
  border-color: #4f7fea;
}

.composer button[type="submit"] {
  padding: 0 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: white;
  background: #386be0;
  font: inherit;
  font-weight: 600;
}

.icon-btn {
  width: 42px;
  height: 42px;
  padding: 0;
  flex-shrink: 0;
  border: 1px solid #343842;
  border-radius: 8px;
  background: #20232a;
  cursor: pointer;
  font-size: 18px;
  line-height: 1;
  color: #f2f3f5;
}

.icon-btn:hover {
  background: #292c34;
}

.composer-emoji-panel {
  position: absolute;
  bottom: calc(100% + 8px);
  right: 20px;
  left: auto;
}
</style>
