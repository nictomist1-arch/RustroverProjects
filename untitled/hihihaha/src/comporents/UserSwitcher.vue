<script setup lang="ts">
import type { User } from "../types/user.ts";

defineProps<{
  users: User[];
  currentUserId: number;
}>();

const emit = defineEmits<{
  select: [user: User];
}>();

function selectUser(user: User) {
  emit("select", user);
}
</script>

<template>
  <div class="user-switcher">
    <span class="user-switcher__label">Пишет:</span>

    <button
      v-for="user in users"
      :key="user.id"
      type="button"
      class="user-switcher__button"
      :class="{ 'user-switcher__button--active': user.id === currentUserId }"
      @click="selectUser(user)"
    >
      <img :src="user.avatar" :alt="user.name" class="user-switcher__avatar" />
      <span>{{ user.name }}</span>
    </button>
  </div>
</template>

<style scoped>
.user-switcher {
  display: flex;
  align-items: center;
  gap: 6px;
}

.user-switcher__label {
  color: #292c34;
  font-size: 12px;
}

.user-switcher__button {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px 4px 4px;
  border: 1px solid #202332;
  border-radius: 999px;
  cursor: pointer;
  background: #8f96a3;
  color: #111318;
  font: inherit;
  font-size: 12px;
}

.user-switcher__button--active {
  background: #17191f;
  border-color: #343842;
  color: #f2f3f5;
}

.user-switcher__avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  object-fit: cover;
  display: block;
}
</style>
