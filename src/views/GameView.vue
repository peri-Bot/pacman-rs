<template>
  <div class="game-view-container">
    <h1>Pacman Game</h1>
    <div class="canvas-placeholder">
      <!-- Canvas will go here later -->
      <p>Game Canvas Area</p>
      <p>[WASM Game Will Load Here]</p>
    </div>
    <router-link to="/" class="retro-button">Back to Menu</router-link>
  </div>
</template>

<script setup>
// Later, import game logic, canvas setup, Wasm loader
import { onMounted } from 'vue';

onMounted(() => {
  console.log("GameView mounted. Ready to load Pacman!");
  // TODO: Initialize Wasm and game canvas
});
</script>

<style lang="scss" scoped>
.game-view-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 20px;
  text-align: center;
}

h1 {
  font-family: 'Press Start 2P', cursive;
  color: #fabd2f; // Gruvbox yellow
  margin-bottom: 30px;
}

.canvas-placeholder {
  width: 448px; // Example Pacman width
  height: 576px; // Example Pacman height
  border: 3px dashed #83a598; // Gruvbox blue-gray
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin-bottom: 30px;
  background-color: #1d2021; // Slightly lighter dark for canvas area
  color: #a89984; // Gruvbox gray
  font-family: 'Press Start 2P', cursive;
  font-size: 0.8em;

  p {
    margin: 10px;
  }
}

.retro-button {
  // Inherits global style, can add specifics
  margin-top: 20px;
}
</style>
