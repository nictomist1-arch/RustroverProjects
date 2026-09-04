<script setup lang="ts">
import { ref } from "vue";

//defineEmits - сообщает vue какие из событий данные
// компонент имеет право рассылать
const emit = defineEmits<{
  send: [body:string];
}>();

const draft = ref("");

function submitMessage(){
  const body = draft.value.trim();

  if(!body) return;

  emit("send", body)

  draft.value = "";

}
</script>

<template>
  <form
      class="composer"
      @submit.prevent="submitMessage()"
  >
    <input
        v-model="draft"
        type="text"
        placeholder="Ну пиши уже че нить"
        autocapitalize="off"
    />
    <button type="submit">Отправить</button>
  </form>
</template>

<style scoped>

.composer{
  display: flex;
  position: sticky;
  gap: 10px;
  padding: 15px 20px;
  border-top: 1px solid #202332;
}

.composer input{
  flex: 1;
  min-width: 0;
  padding: 11px 13px;
  border: 1px solid #292c34;
  border-radius: 7px;
  outline: none;
  color: #292c34;
  background: silver;
  font: inherit;
}

.composer input:focus{
  border-color: #292c34;
}

.composer button{
  padding: 0 18px;
  border: none;
  border-radius: 7px;
  cursor: pointer;
  color: white;
  background: black;
  font: inherit;
  font-size: 1px;
}


</style>