<script setup>
import { TresCanvas } from '@tresjs/core'
import { SRGBColorSpace, ACESFilmicToneMapping } from 'three'
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import ArcadeScene from '../components/ArcadeScene.vue'
import GameMenu from '../components/GameMenu.vue'
import { useArcadeModel } from '../stores/arcadeModel'

// --- State ---
const showMenu = ref(false)
const showPrompt = ref(false)
const modelLoaded = ref(false)

const router = useRouter()
const { modelScene } = useArcadeModel()

// Redirect to loading screen if model wasn't loaded (direct URL access)
onMounted(() => {
  if (!modelScene.value) {
    router.replace({ name: 'landing' })
    return
  }
  modelLoaded.value = true
})

// --- GL Config ---
const gl = {
  clearColor: '#080810',
  shadows: true,
  alpha: false,
  outputColorSpace: SRGBColorSpace,
  toneMapping: ACESFilmicToneMapping,
  toneMappingExposure: 1.2,
}

function onModelReady() {
  showPrompt.value = true
}

function onZoomComplete() {
  showPrompt.value = false
  showMenu.value = true
}
</script>

<template>
  <div class="scene-container">
    <!-- 3D Scene -->
    <TresCanvas v-if="modelLoaded" v-bind="gl" window-size>
      <ArcadeScene @model-ready="onModelReady" @zoom-complete="onZoomComplete" />
    </TresCanvas>

    <!-- "Press SPACE or Click" prompt -->
    <Transition name="prompt-fade">
      <div v-if="showPrompt" class="start-prompt">
        <span class="prompt-text">PRESS SPACE</span>
      </div>
    </Transition>

    <!-- PvP Menu Overlay -->
    <Transition name="menu-fade">
      <GameMenu v-if="showMenu" />
    </Transition>
  </div>
</template>

<style>
html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

#app {
  height: 100%;
  width: 100%;
  background-color: #080810;
}

.scene-container {
  position: relative;
  width: 100%;
  height: 100%;
}

/* Start prompt */
.start-prompt {
  position: absolute;
  bottom: 12%;
  left: 0;
  right: 0;
  text-align: center;
  pointer-events: none;
  z-index: 10;
}

.prompt-text {
  font-family: 'Press Start 2P', monospace;
  font-size: 1rem;
  color: #ffcc00;
  text-shadow:
    0 0 8px rgba(255, 204, 0, 0.8),
    0 0 20px rgba(255, 204, 0, 0.4);
  animation: prompt-blink 1.2s ease-in-out infinite;
  letter-spacing: 2px;
}

@keyframes prompt-blink {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}

/* Prompt fade transition */
.prompt-fade-enter-active {
  transition: opacity 0.6s ease-out;
}
.prompt-fade-leave-active {
  transition: opacity 0.3s ease-in;
}
.prompt-fade-enter-from,
.prompt-fade-leave-to {
  opacity: 0;
}

/* Menu fade-in transition */
.menu-fade-enter-active {
  transition: opacity 0.8s ease-out;
}

.menu-fade-leave-active {
  transition: opacity 0.4s ease-in;
}

.menu-fade-enter-from,
.menu-fade-leave-to {
  opacity: 0;
}
</style>
