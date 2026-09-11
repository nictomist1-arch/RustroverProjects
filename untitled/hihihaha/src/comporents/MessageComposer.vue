<script setup lang="ts">
import { nextTick, ref, useTemplateRef } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";
import EmojiPanel from "../emoji/EmojiPanel.vue";

const emit = defineEmits<{
  send: [body: string];
  sendSticker: [src: string];
  selectEmoji: [emoji: string];
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
  emit("selectEmoji", emoji);

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

async function openImagePicker() {
  const path = await open({
    multiple: false,
    filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "webp"] }],
  });

  if (typeof path !== "string") return;

  const bytes = await readFile(path);
  const type = path.endsWith(".png")
    ? "image/png"
    : path.endsWith(".webp")
      ? "image/webp"
      : "image/jpeg";
  const blob = new Blob([bytes], { type });

  const src = await new Promise<string>((resolve) => {
    const reader = new FileReader();
    reader.onload = () => resolve(String(reader.result));
    reader.readAsDataURL(blob);
  });

  emit("sendSticker", src);
}
</script>

<template>
  <form class="composer" @submit.prevent="submitMessage()">
    <div class="input-group">
      <button
          type="button"
          class="icon-btn"
          title="Прикрепить файл"
          @mousedown.prevent
          @click="openImagePicker"
      >
        📎
      </button>
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

.composer input[type="text"] {
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

.icon-btn {
  padding: 0 14px;
  flex-shrink: 0;
  background: #292c34;
  font-size: 18px;
  line-height: 1;
}

.icon-btn:hover {
  background: #343842;
}

.composer-emoji-panel {
  position: absolute;
  bottom: calc(100% + 8px);
  right: 20px;
  left: auto;
}
</style>
