<script setup lang="ts">
import { computed, ref } from "vue";

import { getFileUrl } from "../types/file.ts";

import type { Message } from "../types/message.ts";

import ImageLightbox from "./ImageLightbox.vue";

const props = defineProps<{
  message: Message;
  isOwn: boolean;
}>();

const isLightboxOpen = ref(false);

const imageSrc = computed(() => {
  if (!props.message.attachment) return null;
  return getFileUrl(props.message.attachment);
});

function openLightbox() {
  if (!imageSrc.value) return;
  isLightboxOpen.value = true;
}
</script>

<template>
  <article
      class="message"
      :class="{
        'message--own': isOwn,
        'message--other': !isOwn,
      }"
  >
    <p
      v-if="
        message.type==='text'
      "
    >
      {{message.body}}
    </p>

    <button
        v-if="
          message.type === 'image'
          &&
          imageSrc
        "
        type="button"
        class="message-image-button"
        @click="openLightbox"
    >
      <img
          class="message-image"
          :src="imageSrc"
          alt=""
      />
    </button>
    <footer>
            <span>
              {{ message.author_name}}
            </span>
      <span>
              |
            </span>
      <span>
              {{message.created_at}}
            </span>
    </footer>
  </article>

  <ImageLightbox
      v-if="isLightboxOpen && imageSrc"
      :src="imageSrc"
      @close="isLightboxOpen = false"
  />
</template>

<style scoped>

.message-image-button{
  display: block;
  padding: 0;
  border: none;
  background: transparent;
  cursor: zoom-in;
}

.message-image{
  display: block;
  max-width: 300px;
  max-height: 300px;
  border-radius: 12px;
  object-fit: cover;
}

.message{
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
}
.message--own{
  align-self: flex-end;
  background: #386be0;
}
.message--other{
  align-self: flex-start;
  background: #252830;
}

.message p{
  margin: 0;
  line-height: 1.45;
  overflow-wrap: anywhere;
}

.message footer{
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 6px;
  color: #b5bbc7;
  font-size: 10px;
}

</style>