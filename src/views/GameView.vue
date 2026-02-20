<template>
  <div class="game-view-container">
    <h1>Pacman Game</h1>

    <!-- WASM status -->
    <div class="wasm-status" :class="{ loaded: wasmLoaded, error: wasmError }">
      <p v-if="wasmError" class="error-text">⚠️ {{ wasmError }}</p>
      <p v-else-if="!wasmLoaded" class="loading-text">⏳ Loading WASM...</p>
      <p v-else class="success-text">{{ greeting }}</p>
    </div>

    <!-- Game state debug panel (Phase 2 verification) -->
    <div v-if="gameState" class="debug-panel">
      <h2>Game State Debug</h2>
      <div class="debug-grid">
        <div class="debug-item">
          <span class="debug-label">Mode</span>
          <span class="debug-value">{{ gameState.mode }}</span>
        </div>
        <div class="debug-item">
          <span class="debug-label">Phase</span>
          <span class="debug-value">{{ gameState.phase }}</span>
        </div>
        <div class="debug-item">
          <span class="debug-label">Level</span>
          <span class="debug-value">{{ gameState.level }}</span>
        </div>
        <div class="debug-item">
          <span class="debug-label">Lives</span>
          <span class="debug-value">{{ gameState.pacman?.lives }}</span>
        </div>
        <div class="debug-item">
          <span class="debug-label">Score</span>
          <span class="debug-value">{{ gameState.pacman?.score }}</span>
        </div>
        <div class="debug-item">
          <span class="debug-label">Dots Left</span>
          <span class="debug-value">{{ gameState.dots_remaining }}</span>
        </div>
        <div class="debug-item">
          <span class="debug-label">Ghosts</span>
          <span class="debug-value">{{ gameState.ghosts?.length }}</span>
        </div>
        <div class="debug-item">
          <span class="debug-label">Maze</span>
          <span class="debug-value">
            {{ gameState.maze?.width }}×{{ gameState.maze?.height }}
          </span>
        </div>
      </div>
    </div>

    <!-- Canvas placeholder -->
    <div class="canvas-placeholder">
      <p>Game Canvas Area</p>
      <p>[Rendering coming in Phase 3]</p>
    </div>

    <router-link to="/" class="retro-button">Back to Menu</router-link>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

// Import the WASM module — wasm-pack generates this in game/pkg/
// `init` is the async initialization function, `greet` and `GameState` are exports
import init, { greet, GameState } from '../../game/pkg'

const wasmLoaded = ref(false)
const wasmError = ref(null)
const greeting = ref('')
const gameState = ref(null)

onMounted(async () => {
  try {
    // Initialize the WASM module (loads and compiles the .wasm binary)
    await init()

    // Phase 1: verify JS↔Rust communication
    greeting.value = greet('Pac-Man')
    wasmLoaded.value = true

    // Phase 2: create a game state and serialize it to JS
    const state = new GameState('classic')
    gameState.value = state.to_js()
    console.log('🦀 WASM loaded! Game state:', gameState.value)
  } catch (err) {
    console.error('Failed to initialize WASM:', err)
    wasmError.value = err.message || 'Unknown WASM error'
  }
})
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
  gap: 20px;
}

h1 {
  font-family: 'Press Start 2P', cursive;
  color: #fabd2f;
  margin-bottom: 10px;
}

// ─── WASM Status Badge ───────────────────────────────────────────────────
.wasm-status {
  padding: 12px 24px;
  border-radius: 8px;
  font-family: 'Press Start 2P', cursive;
  font-size: 0.7em;
  border: 2px solid #555;
  background: #1d2021;

  &.loaded {
    border-color: #b8bb26;
    background: rgba(184, 187, 38, 0.1);
  }

  &.error {
    border-color: #fb4934;
    background: rgba(251, 73, 52, 0.1);
  }
}

.success-text {
  color: #b8bb26;
}

.loading-text {
  color: #fabd2f;
  animation: blink 1.2s ease-in-out infinite;
}

.error-text {
  color: #fb4934;
}

@keyframes blink {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.3;
  }
}

// ─── Debug Panel ────────────────────────────────────────────────────────
.debug-panel {
  width: 448px;
  border: 2px solid #83a598;
  border-radius: 8px;
  padding: 16px;
  background: #1d2021;

  h2 {
    font-family: 'Press Start 2P', cursive;
    font-size: 0.65em;
    color: #83a598;
    margin-bottom: 12px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }
}

.debug-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.debug-item {
  display: flex;
  justify-content: space-between;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 4px;
}

.debug-label {
  font-family: 'Press Start 2P', cursive;
  font-size: 0.5em;
  color: #a89984;
}

.debug-value {
  font-family: 'Press Start 2P', cursive;
  font-size: 0.5em;
  color: #ebdbb2;
}

// ─── Canvas Placeholder ───────────────────────────────────────────────
.canvas-placeholder {
  width: 448px;
  height: 200px;
  border: 3px dashed #504945;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: #1d2021;
  color: #665c54;
  font-family: 'Press Start 2P', cursive;
  font-size: 0.65em;
  border-radius: 8px;

  p {
    margin: 8px;
  }
}

.retro-button {
  margin-top: 10px;
}
</style>
