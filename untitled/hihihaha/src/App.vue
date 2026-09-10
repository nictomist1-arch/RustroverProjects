<script setup lang="ts">

import type { User } from "./types/user";

import type {Message, Reaction, ReactionGroup} from "./types/message.ts";
// Импорт 2 функций из vue
// onMounted - запускает код после появления компонентов
import { onMounted, ref } from "vue";

import Database from "@tauri-apps/plugin-sql";

import AppHeader from "./comporents/AppHeader.vue";

import MessageComposer from "./comporents/MessageComposer.vue";

import MessageList from "./comporents/MessageList.vue";

const oleg: User = {
  id:1,
  name: "Олег",
};

const kirill: User = {
  id:2,
  name: "Кирилл",
};

const fekla: User = {
  id:3,
  name: "Ф Свекла",
};

const users: User[] =[
  oleg,
  kirill,
  fekla,
];

const currentUser = ref<User>(oleg);

async function selectUser(user: User){
  currentUser.value = user;
  await loadMessages();
}

const messages = ref<Message[]>([]);

const status = ref("Подключение...")

let db: Database | null = null;

function groupReactions(
    rows: Reaction[],
    messageId: number,
    userName: string,
): ReactionGroup[] {
  const byEmoji = new Map<string, ReactionGroup>();

  for (const row of rows) {
    if (row.message_id !== messageId) continue;

    const existing = byEmoji.get(row.emoji);
    if (existing) {
      existing.count += 1;
      if (row.author === userName) existing.reactedByMe = true;
    } else {
      byEmoji.set(row.emoji, {
        emoji: row.emoji,
        count: 1,
        reactedByMe: row.author === userName,
      });
    }
  }

  return Array.from(byEmoji.values());
}

async function loadMessages(){
  if(!db) return;

  const rows = await db.select<Omit<Message, "reactions">[]>(
      "SELECT id, author, body, created_at FROM messages ORDER BY id ASC",
  );

  const reactionRows = await db.select<Reaction[]>(
      "SELECT id, message_id, author, emoji FROM reactions ORDER BY id ASC",
  );

  const userName = currentUser.value.name;

  messages.value = rows.map((row) => ({
    ...row,
    reactions: groupReactions(reactionRows, row.id, userName),
  }));
}

async function sendMessage(body: string) {
  if (!db) return;

  await db.execute(
      "INSERT INTO messages (author, body) VALUES ($1, $2)",
      [
        currentUser.value.name,
        body,
      ]
  )
  await loadMessages()
}

async function toggleReaction(messageId: number, emoji: string) {
  if (!db) return;

  const author = currentUser.value.name;

  const existing = await db.select<Reaction[]>(
      "SELECT id, message_id, author, emoji FROM reactions WHERE message_id = $1 AND author = $2 AND emoji = $3",
      [messageId, author, emoji],
  );

  if (existing.length > 0) {
    await db.execute("DELETE FROM reactions WHERE id = $1", [existing[0].id]);
  } else {
    await db.execute(
        "INSERT INTO reactions (message_id, author, emoji) VALUES ($1, $2, $3)",
        [messageId, author, emoji],
    );
  }

  await loadMessages();
}

// VUE выполнит код ниже когда интерфейс программы загрузится
onMounted(async ()=> {
  try {
    // Открываем бд
    db = await Database.load("sqlite:messanger.db");

    await loadMessages();

    status.value = "История сохраняется локально";
  } catch (error) {
    console.error(error);
    status.value = "Ошибка подключения к базе";
  }
});

</script>

<template>
  <main class="App">
    <AppHeader
        :status="status"
        :users="users"
        :current-user="currentUser"
        @select="selectUser"
    />
    <section class="chat">
      <div class="chat-info">
        <h2>Первый чат</h2>

        <p>Первый локальной мессенджер</p>
      </div>
      <MessageList
          :messages="messages"
          :current-user-name="currentUser.name"
          @react="toggleReaction"
      />
      <div class="composer-wrapper">
        <MessageComposer @send="sendMessage"/>
      </div>
    </section>
  </main>
</template>

<style scoped>
:global(*){
  box-sizing: border-box;
}

:global(html){
  background: #111318;
  color-scheme: dark;
}

:global(body){
  margin: 0;

  font-family:
      Inter,
      system-ui,
      -apple-system,
      BlinkMacSystemFontm,
      "Segoe UI",
      sans-serif;

  color: #f2f3f5;

  background: #111318;
}

.App{
  height: 100vh;
  display: flex;
  flex-direction: column;
  /* Запрещает всему app прокручиваться */
  overflow: hidden;
}

.chat{
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden; /* Потому что chat целиком не должен прокручиваться  */
}

.chat-info{
  padding: 20px 24px;
  border-bottom: 1px solid #8f96a3;
}

.chat-info h2{
  margin: 0;
  font-size: 16px;
}

.chat-info p{
  margin: 5px 0 0;
  color: #292c34;
  font-size: 13px;
}

</style>
