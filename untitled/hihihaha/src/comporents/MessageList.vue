<script setup lang="ts">
import MessageBubble from "./MessageBubble.vue";

import type {Message} from "../types/message.ts";

defineProps<{
  messages: Message[];
}>();
</script>

<template>
  <div class="messages">
    <!-- Данный див будет отображаться когда сообщений нет -->
    <div
        v-if="messages.length === 0"
        class="empty"
    >
      <strong> Здесь пока пусто </strong>
      <span> Напишите первое сообщение </span>
    </div>
    <!-- Vue создает article ля каждого сообщения из базы -->
    <MessageBubble
    v-for="message in messages"
    :key="message.id"
    :message="message"
    />
  </div>
</template>

<style scoped>

.messages{
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 24px;
}

.empty{
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: center;
  color: #858c98;
}

</style>