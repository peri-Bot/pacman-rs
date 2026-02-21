<script setup>
import { useRouter } from 'vue-router'

const router = useRouter()

function startClassic() {
  router.push({ name: 'play', query: { mode: 'classic' } })
}

function startPvP() {
  router.push({ name: 'play', query: { mode: 'pvp' } })
}

function showCredits() {
  // For now just alert — will be a modal later
  alert(
    'This work is based on "Pacman Arcade + animation" by Daniel Brück (https://sketchfab.com/daniel.brueck) licensed under CC-BY-4.0 (http://creativecommons.org/licenses/by/4.0/)\n\nPowered by Rust + WebAssembly'
  )
}
</script>

<template>
  <div class="game-menu-overlay">
    <!-- CRT Scanline Effect -->
    <div class="scanlines"></div>

    <div class="menu-content">
      <!-- Title -->
      <div class="title-section">
        <h1 class="game-title">PAC-MAN</h1>
      </div>

      <!-- Animated Pac-Man & Ghost -->
      <div class="pac-chase">
        <div class="pac-dot"></div>
        <div class="pac-dot"></div>
        <div class="pac-dot"></div>
        <div class="chase-pacman"></div>
        <div class="chase-ghost ghost-red"></div>
        <div class="chase-ghost ghost-pink"></div>
        <div class="chase-ghost ghost-cyan"></div>
        <div class="chase-ghost ghost-orange"></div>
      </div>

      <!-- Menu Buttons -->
      <div class="menu-buttons">
        <button class="menu-btn primary" @click="startClassic">
          <span class="btn-icon">🟡</span>
          <span class="btn-text">CLASSIC MODE</span>
        </button>
        <button class="menu-btn secondary" @click="startPvP">
          <span class="btn-icon">👾</span>
          <span class="btn-text">MULTIPLAYER (1 VS 1)</span>
        </button>
        <button class="menu-btn tertiary" @click="showCredits">
          <span class="btn-icon">ℹ️</span>
          <span class="btn-text">CREDITS</span>
        </button>
      </div>

      <!-- Insert Coin -->
      <div class="insert-coin">INSERT COIN</div>

      <!-- Bottom Credit -->
      <div class="powered-by">POWERED BY RUST + WASM</div>
    </div>
  </div>
</template>

<style scoped>
.game-menu-overlay {
  position: absolute;
  top: 50rem;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  background: radial-gradient(ellipse at center, rgba(0, 0, 30, 0.85) 0%, rgba(0, 0, 0, 0.95) 70%);
}

/* CRT Scanline overlay */
.scanlines {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 101;
  background: repeating-linear-gradient(to bottom,
      transparent 0px,
      transparent 2px,
      rgba(0, 0, 0, 0.15) 2px,
      rgba(0, 0, 0, 0.15) 4px);
}

.menu-content {
  position: relative;
  z-index: 102;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 28px;
  padding: 40px 50px;
  max-width: 500px;
  width: 90%;
}

/* Title */
.title-section {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 8px;
}

.game-title {
  font-family: 'Press Start 2P', cursive;
  font-size: 3rem;
  color: #FFD700;
  text-shadow:
    0 0 10px #FFD700,
    0 0 30px #FF8C00,
    0 0 60px #FF4500,
    3px 3px 0px #CC0000;
  margin: 0;
  letter-spacing: 0.05em;
  animation: titleGlow 2s ease-in-out infinite alternate;
}

@keyframes titleGlow {
  from {
    text-shadow:
      0 0 10px #FFD700,
      0 0 30px #FF8C00,
      0 0 60px #FF4500,
      3px 3px 0px #CC0000;
  }

  to {
    text-shadow:
      0 0 15px #FFEE44,
      0 0 40px #FFAA00,
      0 0 80px #FF6600,
      3px 3px 0px #CC0000;
  }
}

.pvp-badge {
  font-family: 'Press Start 2P', cursive;
  font-size: 1.2rem;
  color: #00FF00;
  border: 2px solid #00FF00;
  padding: 6px 12px;
  border-radius: 4px;
  text-shadow: 0 0 10px #00FF00;
  box-shadow:
    0 0 10px rgba(0, 255, 0, 0.3),
    inset 0 0 10px rgba(0, 255, 0, 0.1);
  animation: pvpPulse 1.5s ease-in-out infinite alternate;
}

@keyframes pvpPulse {
  from {
    box-shadow: 0 0 10px rgba(0, 255, 0, 0.3), inset 0 0 10px rgba(0, 255, 0, 0.1);
  }

  to {
    box-shadow: 0 0 20px rgba(0, 255, 0, 0.6), inset 0 0 20px rgba(0, 255, 0, 0.2);
  }
}

/* Pac-Man Chase Animation */
.pac-chase {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 0;
  overflow: hidden;
  width: 100%;
  justify-content: center;
  animation: chaseMove 6s linear infinite;
}

@keyframes chaseMove {
  0% {
    transform: translateX(-30px);
  }

  50% {
    transform: translateX(30px);
  }

  100% {
    transform: translateX(-30px);
  }
}

.pac-dot {
  width: 6px;
  height: 6px;
  background-color: #FFD700;
  border-radius: 50%;
  animation: dotBlink 0.5s ease-in-out infinite alternate;
}

.chase-pacman {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  background: conic-gradient(
    from 270deg,
    transparent 0deg 35deg,
    #FFD700 35deg 325deg,
    transparent 325deg 360deg
  );
  position: relative;
  animation: chomp 0.3s ease-in-out infinite alternate;
}

@keyframes chomp {
  from {
    background: conic-gradient(
      from 270deg,
      transparent 0deg 5deg,
      #FFD700 5deg 355deg,
      transparent 355deg 360deg
    );
  }

  to {
    background: conic-gradient(
      from 270deg,
      transparent 0deg 35deg,
      #FFD700 35deg 325deg,
      transparent 325deg 360deg
    );
  }
}

.chase-ghost {
  width: 26px;
  height: 28px;
  border-radius: 13px 13px 4px 4px;
  position: relative;
  animation: ghostFloat 0.4s ease-in-out infinite alternate;
}

.chase-ghost::before,
.chase-ghost::after {
  content: '';
  position: absolute;
  background-color: white;
  width: 7px;
  height: 8px;
  border-radius: 50%;
  top: 7px;
}

.chase-ghost::before {
  left: 4px;
}

.chase-ghost::after {
  right: 4px;
}

.ghost-red {
  background-color: #FF0000;
}

.ghost-pink {
  background-color: #FFB8FF;
}

.ghost-cyan {
  background-color: #00FFFF;
}

.ghost-orange {
  background-color: #FFB852;
}

@keyframes ghostFloat {
  from {
    transform: translateY(-2px);
  }

  to {
    transform: translateY(2px);
  }
}

@keyframes dotBlink {
  from {
    opacity: 1;
  }

  to {
    opacity: 0.3;
  }
}

/* Menu Buttons */
.menu-buttons {
  display: flex;
  flex-direction: column;
  gap: 14px;
  width: 100%;
  max-width: 320px;
}

.menu-btn {
  font-family: 'Press Start 2P', cursive;
  font-size: 0.95rem;
  padding: 16px 24px;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 14px;
  justify-content: center;
  transition: all 0.15s ease;
  position: relative;
  overflow: hidden;
}

.menu-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transition: left 0.5s ease;
}

.menu-btn:hover::before {
  left: 100%;
}

.menu-btn.primary {
  background: linear-gradient(135deg, #FF4444 0%, #CC0000 100%);
  color: #FFD700;
  border: 2px solid #FF6666;
  box-shadow: 0 0 20px rgba(255, 68, 68, 0.4), 0 4px 12px rgba(0, 0, 0, 0.5);
}

.menu-btn.primary:hover {
  background: linear-gradient(135deg, #FF6666 0%, #EE2222 100%);
  transform: scale(1.03);
  box-shadow: 0 0 30px rgba(255, 68, 68, 0.6), 0 6px 16px rgba(0, 0, 0, 0.5);
}

.menu-btn.primary:active {
  transform: scale(0.97);
  box-shadow: 0 0 10px rgba(255, 68, 68, 0.3), 0 2px 6px rgba(0, 0, 0, 0.5);
}

.menu-btn.secondary {
  background: linear-gradient(135deg, #2233AA 0%, #1A1A66 100%);
  color: #88BBFF;
  border: 2px solid #3344CC;
  box-shadow: 0 0 15px rgba(34, 51, 170, 0.3), 0 4px 12px rgba(0, 0, 0, 0.5);
}

.menu-btn.secondary:hover {
  background: linear-gradient(135deg, #3344CC 0%, #2233AA 100%);
  transform: scale(1.03);
}

.menu-btn.tertiary {
  background: linear-gradient(135deg, #333333 0%, #1a1a1a 100%);
  color: #888888;
  border: 2px solid #444444;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
}

.menu-btn.tertiary:hover {
  background: linear-gradient(135deg, #444444 0%, #2a2a2a 100%);
  color: #bbbbbb;
  transform: scale(1.03);
}

.btn-icon {
  font-size: 1.2rem;
  font-style: normal;
}

.btn-text {
  letter-spacing: 0.1em;
}

/* Insert Coin */
.insert-coin {
  font-family: 'Press Start 2P', cursive;
  font-size: 0.85rem;
  color: #00FFFF;
  animation: blink 1.2s ease-in-out infinite;
  text-shadow: 0 0 10px #00FFFF;
  margin-top: 8px;
}

@keyframes blink {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.2;
  }
}

/* Powered By */
.powered-by {
  font-family: 'Press Start 2P', cursive;
  font-size: 0.55rem;
  color: #444;
  letter-spacing: 0.15em;
  margin-top: 4px;
}
</style>
