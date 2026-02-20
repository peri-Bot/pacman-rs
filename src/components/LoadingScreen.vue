<template>
  <div class="loading-container">
    <div class="loading-content">
      <div class="pacman-logo">PAC-MAN</div>

      <div class="ghost-parade">
        <div class="loading-ghost ghost-red"></div>
        <div class="loading-ghost ghost-pink"></div>
        <div class="loading-ghost ghost-cyan"></div>
        <div class="loading-ghost ghost-orange"></div>
        <div class="loading-pacman"></div>
      </div>

      <!-- Progress Bar -->
      <div class="progress-wrapper">
        <div class="progress-track">
          <div class="progress-fill" :style="{ width: `${loadingProgress}%` }"></div>
          <div class="progress-pacman" :style="{ left: `${loadingProgress}%` }"></div>
        </div>
        <div class="progress-label">{{ loadingProgress }}%</div>
      </div>

      <div class="loading-text">{{ loadingStatus }}</div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js'
import { useArcadeModel } from '../stores/arcadeModel'

const loadingProgress = ref(0)
const loadingStatus = ref('LOADING ARCADE...')
const router = useRouter()
const { setModel } = useArcadeModel()

let tickId = null
let realProgress = 0
let modelReady = false
const MIN_DISPLAY_MS = 1500

function startProgressTick(startTime) {
  tickId = setInterval(() => {
    // Smoothly approach the real progress
    const target = modelReady ? 100 : Math.min(realProgress, 95)
    const diff = target - loadingProgress.value

    if (diff > 0) {
      // Move toward target by a fraction each tick — feels smooth
      loadingProgress.value = Math.round(loadingProgress.value + Math.max(diff * 0.15, 0.5))
    }

    if (loadingProgress.value >= 100 && modelReady) {
      loadingProgress.value = 100
      loadingStatus.value = 'READY!'
      clearInterval(tickId)
      tickId = null

      // Ensure minimum display time
      const elapsed = Date.now() - startTime
      const remaining = Math.max(0, MIN_DISPLAY_MS - elapsed)
      setTimeout(() => {
        router.push({ name: 'ArcadeMachine' })
      }, remaining + 400)
    }
  }, 50)
}

onMounted(() => {
  const startTime = Date.now()
  const loader = new GLTFLoader()

  startProgressTick(startTime)

  loader.load(
    '/pacman_arcade/scene.gltf',
    (gltf) => {
      setModel(gltf)
      realProgress = 100
      modelReady = true
    },
    (event) => {
      if (event.total > 0) {
        realProgress = Math.round((event.loaded / event.total) * 100)
      } else {
        realProgress = Math.min(realProgress + 2, 90)
      }
    },
    (error) => {
      console.error('Failed to load arcade model:', error)
      loadingStatus.value = 'ERROR LOADING...'
      clearInterval(tickId)
    },
  )
})

onUnmounted(() => {
  if (tickId) clearInterval(tickId)
})
</script>

<style scoped>
.loading-container {
  width: 100%;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: radial-gradient(ellipse at center, #0a0a2e 0%, #000000 70%);
  overflow: hidden;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 32px;
}

.pacman-logo {
  font-family: 'Press Start 2P', cursive;
  font-size: 3.5rem;
  color: #ffd700;
  text-shadow:
    0 0 10px #ffd700,
    0 0 30px #ff8c00,
    0 0 60px #ff4500,
    3px 3px 0px #cc0000;
  animation: logoGlow 2s ease-in-out infinite alternate;
}

@keyframes logoGlow {
  from {
    filter: brightness(1);
  }
  to {
    filter: brightness(1.3);
  }
}

/* Ghost Parade */
.ghost-parade {
  display: flex;
  align-items: center;
  gap: 12px;
  animation: paradeFloat 4s ease-in-out infinite;
}

@keyframes paradeFloat {
  0%,
  100% {
    transform: translateX(-20px);
  }
  50% {
    transform: translateX(20px);
  }
}

.loading-ghost {
  width: 30px;
  height: 34px;
  border-radius: 15px 15px 4px 4px;
  position: relative;
  animation: ghostBob 0.5s ease-in-out infinite alternate;
}

.loading-ghost::before,
.loading-ghost::after {
  content: '';
  position: absolute;
  background-color: white;
  width: 8px;
  height: 9px;
  border-radius: 50%;
  top: 8px;
}

.loading-ghost::before {
  left: 5px;
}
.loading-ghost::after {
  right: 5px;
}

.ghost-red {
  background-color: #ff0000;
  animation-delay: 0s;
}
.ghost-pink {
  background-color: #ffb8ff;
  animation-delay: 0.1s;
}
.ghost-cyan {
  background-color: #00ffff;
  animation-delay: 0.2s;
}
.ghost-orange {
  background-color: #ffb852;
  animation-delay: 0.3s;
}

@keyframes ghostBob {
  from {
    transform: translateY(-3px);
  }
  to {
    transform: translateY(3px);
  }
}

.loading-pacman {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: conic-gradient(
    from 90deg,
    transparent 0deg 35deg,
    #ffd700 35deg 325deg,
    transparent 325deg 360deg
  );
  animation: loadChomp 0.35s ease-in-out infinite alternate;
}

@keyframes loadChomp {
  from {
    background: conic-gradient(
      from 90deg,
      transparent 0deg 5deg,
      #ffd700 5deg 355deg,
      transparent 355deg 360deg
    );
  }
  to {
    background: conic-gradient(
      from 90deg,
      transparent 0deg 35deg,
      #ffd700 35deg 325deg,
      transparent 325deg 360deg
    );
  }
}

/* Progress Bar */
.progress-wrapper {
  width: 350px;
  max-width: 80vw;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.progress-track {
  width: 100%;
  height: 12px;
  background-color: #1a1a3a;
  border-radius: 6px;
  overflow: visible;
  position: relative;
  border: 1px solid #333366;
  box-shadow:
    0 0 10px rgba(0, 0, 0, 0.5),
    inset 0 0 5px rgba(0, 0, 0, 0.5);
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #ffd700, #ff8c00);
  border-radius: 6px;
  transition: width 0.4s ease-out;
  box-shadow: 0 0 10px rgba(255, 215, 0, 0.5);
}

.progress-pacman {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: conic-gradient(
    from 90deg,
    transparent 0deg 30deg,
    #ffd700 30deg 330deg,
    transparent 330deg 360deg
  );
  animation: miniChomp 0.3s ease-in-out infinite alternate;
  transition: left 0.4s ease-out;
}

@keyframes miniChomp {
  from {
    background: conic-gradient(
      from 90deg,
      transparent 0deg 5deg,
      #ffd700 5deg 355deg,
      transparent 355deg 360deg
    );
  }
  to {
    background: conic-gradient(
      from 90deg,
      transparent 0deg 30deg,
      #ffd700 30deg 330deg,
      transparent 330deg 360deg
    );
  }
}

.progress-label {
  font-family: 'Press Start 2P', cursive;
  font-size: 0.75rem;
  color: #ffd700;
  text-shadow: 0 0 5px rgba(255, 215, 0, 0.5);
}

.loading-text {
  font-family: 'Press Start 2P', cursive;
  font-size: 0.8rem;
  color: #00ffff;
  animation: textBlink 1.2s ease-in-out infinite;
  text-shadow: 0 0 10px #00ffff;
}

@keyframes textBlink {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}
</style>
