<!-- src/App.vue -->
<template>
  <LoadingScreen />

  <!-- Your main application content, shown when loading is false -->
  <div v-if="!isLoading" class="main-content">
    <h1>Welcome to Rust-y Pacman!</h1>
    <p>The game will be rendered here.</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import LoadingScreen from './components/LoadingScreen.vue';
import ArcadeMachine from './components/ArcadeMachine.vue';

const isLoading = ref(true);

const handleLoadingComplete = () => {
  isLoading.value = false;
};
</script>

<!-- Styles can remain the same, the fade transition is no longer needed here -->
<style lang="scss">
@import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

body {
  margin: 0;
  background-color: #000;
  color: #fff;
  font-family: 'Press Start 2P', cursive;
}

.main-content {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
  text-align: center;
}
</style>
