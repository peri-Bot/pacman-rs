// src/components/ArcadeScreen.vue
<template>
  <div class="arcade-screen-ui" :class="{ 'game-active': gameHasStarted }">
    <template v-if="!gameHasStarted">
      <!-- <img src="/assets/logo.svg" alt="Pac-Man Logo" class="logo" /> <!-- Assuming logo.svg is your Pac-Man logo -->
      -->
      <div class="title">PAC-MAN</div>
      <button @click="triggerStartGame" class="start-button">START GAME</button>
      <div class="credits">INSERT COIN</div>
    </template>
    <template v-else>
      <!-- This part would be shown if the game itself renders here -->
      <div class="game-placeholder-text">
        LOADING GAME...
        <!-- Your Rust/WASM game canvas/container would go here -->
      </div>
    </template>
  </div>
</template>

<script setup>
import { ref } from 'vue';
const emit = defineEmits(['startGame']);

const gameHasStarted = ref(false); // Local state for screen content

function triggerStartGame() {
  // gameHasStarted.value = true; // Uncomment if game plays on this screen
  emit('startGame'); // Notify parent (PacmanCabinet)
}
</script>

<style scoped>
/* Add the "Press Start 2P" font to your project (e.g., in public/index.html or main.css) */
/* @import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap'); */

.arcade-screen-ui {
  /*
    IMPORTANT: The width/height here should be chosen carefully.
    The `htmlScale` in PacmanCabinet.vue will scale this down.
    Aim for a size that gives good text clarity when scaled.
    E.g., if screenWidth in 3D is 0.6 and htmlScale is 0.002,
    then 0.6 / 0.002 = 300px. So width here could be around 300px.
  */
  width: 270px;
  /* (0.6 / 0.0022) approx */
  height: 202px;
  /* (0.45 / 0.0022) approx */
  background-color: #00000A;
  color: #FFFF00;
  font-family: 'Press Start 2P', cursive;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-around;
  /* Adjusted for better spacing */
  padding: 15px;
  /* Adjusted padding */
  box-sizing: border-box;
  border: 4px solid #1a1a55;
  border-radius: 3px;
  text-align: center;
  overflow: hidden;
  /* Crucial for TresJS Html component */
  image-rendering: pixelated;
  /* Helps with pixel art style */
}

.logo {
  width: 60px;
  /* Adjusted size */
  margin-top: 5px;
}

.title {
  font-size: 22px;
  /* Adjusted size */
  color: #FFFF00;
  text-shadow: 2px 2px #FF0000, -1px -1px #39f;
  line-height: 1.1;
}

.start-button {
  background-color: #FF0000;
  color: #FFFF00;
  border: 2px solid #FFFF00;
  padding: 8px 12px;
  /* Adjusted padding */
  font-family: 'Press Start 2P', cursive;
  font-size: 12px;
  /* Adjusted size */
  cursor: pointer;
  box-shadow: 1px 1px #000;
  text-transform: uppercase;
}

.start-button:hover {
  background-color: #FFFF00;
  color: #FF0000;
  border-color: #FF0000;
}

.credits {
  font-size: 10px;
  /* Adjusted size */
  color: #00FFFF;
  /* Cyan */
  animation: blink-animation 1.2s infinite;
  margin-bottom: 5px;
}

@keyframes blink-animation {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.4;
  }
}

.game-placeholder-text {
  font-size: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}
</style>
