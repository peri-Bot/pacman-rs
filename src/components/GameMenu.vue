<script setup lang="ts">
// This function will eventually be loaded from your WASM module.
// For now, it's just a placeholder.
function startGame() {
  alert('Starting game! (This would call a WASM function)');
  // In a real scenario:
  // import { start_game } from 'my-wasm-package';
  // start_game();
}
</script>

<template>
  <!--
    This component will be rendered off-screen.
    The size here should match the aspect ratio of the 3D screen plane.
    Our screen is 1.6 x 1.26, which is roughly a 4:3.15 aspect ratio.
    Let's use 400px width for decent resolution. 400 * (1.26 / 1.6) = 315.
  -->
  <div class="arcade-menu-container">
    <h1>RUSTY ARCADE</h1>
    <button @click="startGame">START GAME</button>
    <p>High Score: 9999</p>
  </div>
</template>

<style scoped>
.arcade-menu-container {
  width: 400px;
  height: 315px;
  background-color: #001144;
  color: #FFFF99;
  font-family: 'Press Start 2P', cursive;
  /* Add a retro font for style! */
  text-align: center;
  padding: 20px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 20px;
}

h1 {
  color: #FF3333;
  font-size: 2.5rem;
  text-shadow: 3px 3px 0px #282828;
  margin: 0;
}

button {
  font-family: 'Press Start 2P', cursive;
  font-size: 1.5rem;
  padding: 15px 25px;
  background-color: #33FF33;
  color: #282828;
  border: 3px solid #282828;
  cursor: pointer;
  box-shadow: 5px 5px 0px #252525;
  transition: all 0.1s ease-in-out;
}

button:hover {
  background-color: #8CFF8C;
}

button:active {
  transform: translate(5px, 5px);
  box-shadow: none;
}

p {
  font-size: 1.2rem;
  color: white;
}
</style>
