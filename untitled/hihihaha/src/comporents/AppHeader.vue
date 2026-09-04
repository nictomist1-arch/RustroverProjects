<script setup lang="ts">

//defineProps - спец конструкция vue которая сообщает:
// Этот компонент ожидает получение данных от родительского компонента

defineProps<{
  status:string;
}>();

</script>

<template>
  <header class="header">
    <div>
      <h1>Hihihaha</h1>

      <p>{{status}}</p>
    </div>

    <span class="badge">
        Локально
      </span>
  </header>
</template>

<style scoped>
/*
  CSS этого блока будет относится только к текущему vue компоненту
  Например .header не повлиляет на любой другой .header в коде вне этого компонента
 */

.header{
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 24px;
  border-bottom: 1px solid #292c34;
  background: #17191f;
}

.header h1 {
  margin: 0;
  font-size: 18px;
}

.header p{
  margin: 4px 0 0;
  color: #8f96a3;
}

.badge{
  padding: 6px 10px;
  border: 1px solid #343842;
  border-radius: 6px;
  color: aqua;
  background: #202332;
  font-size: 12px;
}

</style>