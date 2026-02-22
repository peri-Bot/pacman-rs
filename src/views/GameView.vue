<template>
  <div class="game-view-container">
    <!-- Game Canvas (Phase 3+6) -->
    <GameCanvas
      v-if="wasmInstance && gameStateData"
      :wasm-instance="wasmInstance"
      :game-state="gameStateData"
    />
  </div>
</template>

<script setup>
import { ref, shallowRef, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import GameCanvas from '../components/GameCanvas.vue'

// Import the WASM module
import init, { GameState } from '../../game/pkg'

const route = useRoute()
const wasmLoaded = ref(false)
const wasmError = ref(null)
const gameStateData = ref(null)
const wasmInstance = shallowRef(null)

onMounted(async () => {
  try {
    await init()
    const mode = route.query.mode || 'classic'
    // Create the game state wrapping instance
    const state = new GameState(mode)
    wasmInstance.value = state

    // Get initial state delta
    gameStateData.value = state.to_js()
    wasmLoaded.value = true
    console.log(`🦀 WASM loaded! Game loaded in ${mode} mode.`)
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
  width: 100vw;
  background-color: #000;
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>
