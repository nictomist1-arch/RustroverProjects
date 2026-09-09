<script setup lang="ts">

import type { User } from "./types/user";

import type {Message} from "./types/message.ts";
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

function selectUser(user: User){
  currentUser.value = user;
}

const messages = ref<Message[]>([]);

const status = ref("Подключение...")

let db: Database | null = null;

async  function loadMessages(){
  if(!db) return;

  messages.value = await db.select<Message[]>(
      "SELECT id, author, body, created_at FROM messages ORDER BY id ASC",
  );
}

async  function sendMessage(body: string) {
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