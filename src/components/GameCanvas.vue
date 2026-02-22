<template>
  <div class="game-board-container">
    <!-- Overlay for HUD (Score, Lives) -->
    <div class="hud">
      <div class="score">SCORE: {{ score }}</div>
      <div class="lives">LIVES: <span v-for="n in lives" :key="n">❤️</span></div>
    </div>

    <canvas ref="canvas" :width="canvasWidth" :height="canvasHeight"></canvas>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const props = defineProps({
  gameState: {
    type: Object,
    required: true,
  },
  wasmInstance: {
    type: Object,
    required: true,
  }
})

const canvas = ref(null)
const ctx = ref(null)

// Standard Pac-Man dimensions: 28 cols x 31 rows.
// Using 20px per tile for a 560x620 board.
const TILE_SIZE = 20
const boardWidth = 28 * TILE_SIZE
const boardHeight = 31 * TILE_SIZE
const RENDER_SCALE = 4
const canvasWidth = boardWidth * RENDER_SCALE
const canvasHeight = boardHeight * RENDER_SCALE

const score = ref(0)
const lives = ref(3)

let animationFrameId = null
let lastTime = 0

onMounted(() => {
  ctx.value = canvas.value.getContext('2d')

  // Setup keyboard controls
  window.addEventListener('keydown', handleKeyDown)

  // Start loop
  lastTime = performance.now()
  animationFrameId = requestAnimationFrame(gameLoop)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId)
  }
})

function handleKeyDown(e) {
  let p1Dir = null
  let p2Dir = null

  if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
    // Player 1 (Arrow keys)
    switch(e.key) {
      case 'ArrowUp': p1Dir = 'up'; break;
      case 'ArrowDown': p1Dir = 'down'; break;
      case 'ArrowLeft': p1Dir = 'left'; break;
      case 'ArrowRight': p1Dir = 'right'; break;
    }
  } else if (['w', 'a', 's', 'd', 'W', 'A', 'S', 'D'].includes(e.key)) {
    // Player 2 / Alternative P1 (WASD keys)
    switch(e.key.toLowerCase()) {
      case 'w': p2Dir = 'up'; break;
      case 's': p2Dir = 'down'; break;
      case 'a': p2Dir = 'left'; break;
      case 'd': p2Dir = 'right'; break;
    }
  }

  // If we are in PvP mode, Player 2 uses WASD
  if (props.gameState.mode === 'pvp') {
    if (p1Dir) {
      e.preventDefault()
      props.wasmInstance.set_direction(p1Dir)
    }
    if (p2Dir) {
      e.preventDefault()
      props.wasmInstance.set_player2_direction(p2Dir)
    }
  } else {
    // In classic mode, both WASD and Arrows control Pacman
    let dir = p1Dir || p2Dir
    if (dir) {
      e.preventDefault()
      props.wasmInstance.set_direction(dir)
    }
  }
}

function gameLoop(time) {
  const dt = time - lastTime
  lastTime = time

  // Advance simulation (WASM tick using dt)
  props.wasmInstance.tick(dt)

  // Read the new state (delta sync)
  const state = props.wasmInstance.to_js()

  // Update HUD reactives
  score.value = state.pacman.score
  lives.value = state.pacman.lives

  // Render frame
  render(state)

  animationFrameId = requestAnimationFrame(gameLoop)
}

function render(state) {
  if (!ctx.value) return

  // 1. Clear canvas
  ctx.value.fillStyle = '#000000'
  ctx.value.fillRect(0, 0, canvasWidth, canvasHeight)

  ctx.value.save()
  ctx.value.scale(RENDER_SCALE, RENDER_SCALE)

  // 2. Draw Maze
  // Grid layout mapping: Empty, Wall, Dot, PowerPellet, GhostHouse
  const maze = state.maze.cells
  for (let r = 0; r < state.maze.height; r++) {
    for (let c = 0; c < state.maze.width; c++) {
      const cell = maze[r][c]
      const x = c * TILE_SIZE
      const y = r * TILE_SIZE

      if (cell === 'Wall') {
        const isWall = (row, col) => row >= 0 && row < state.maze.height && col >= 0 && col < state.maze.width && maze[row][col] === 'Wall';

        // Base solid blue
        ctx.value.fillStyle = '#2222FF';
        ctx.value.fillRect(x, y, TILE_SIZE, TILE_SIZE);

        // Draw black inner block to carve out the hollow space
        ctx.value.fillStyle = '#000000';
        const pad = 4;
        let bx = x + pad;
        let by = y + pad;
        let bw = TILE_SIZE - pad * 2;
        let bh = TILE_SIZE - pad * 2;

        // Extend black space into adjacent walls to merge the hollow paths
        if (isWall(r - 1, c)) { by -= pad; bh += pad; }
        if (isWall(r + 1, c)) { bh += pad; }
        if (isWall(r, c - 1)) { bx -= pad; bw += pad; }
        if (isWall(r, c + 1)) { bw += pad; }

        ctx.value.fillRect(bx, by, bw, bh);

        // Restore blue inner corners if the diagonal is open (concave corners)
        ctx.value.fillStyle = '#2222FF';
        // Top-Left inner corner
        if (isWall(r - 1, c) && isWall(r, c - 1) && !isWall(r - 1, c - 1)) {
            ctx.value.fillRect(x, y, pad, pad);
        }
        // Top-Right inner corner
        if (isWall(r - 1, c) && isWall(r, c + 1) && !isWall(r - 1, c + 1)) {
            ctx.value.fillRect(x + TILE_SIZE - pad, y, pad, pad);
        }
        // Bottom-Left inner corner
        if (isWall(r + 1, c) && isWall(r, c - 1) && !isWall(r + 1, c - 1)) {
            ctx.value.fillRect(x, y + TILE_SIZE - pad, pad, pad);
        }
        // Bottom-Right inner corner
        if (isWall(r + 1, c) && isWall(r, c + 1) && !isWall(r + 1, c + 1)) {
            ctx.value.fillRect(x + TILE_SIZE - pad, y + TILE_SIZE - pad, pad, pad);
        }
      } else if (cell === 'Dot') {
        ctx.value.fillStyle = '#FFB8AE' // Peach dots
        ctx.value.beginPath()
        ctx.value.arc(x + TILE_SIZE / 2, y + TILE_SIZE / 2, 2, 0, Math.PI * 2)
        ctx.value.fill()
      } else if (cell === 'PowerPellet') {
        // Blink power pellets
        if (Math.floor(performance.now() / 250) % 2 === 0) {
          ctx.value.fillStyle = '#FFB8AE'
          ctx.value.beginPath()
          ctx.value.arc(x + TILE_SIZE / 2, y + TILE_SIZE / 2, 5, 0, Math.PI * 2)
          ctx.value.fill()
        }
      }
    }
  }

  // Ghost house door
  ctx.value.fillStyle = '#FFC0CB'
  ctx.value.fillRect(13 * TILE_SIZE, 12 * TILE_SIZE + 10, TILE_SIZE * 2, 4)

  // 3. Draw Pac-Man
  const px = state.pacman.position.x * TILE_SIZE
  const py = state.pacman.position.y * TILE_SIZE

  ctx.value.fillStyle = '#FFFF00'
  ctx.value.beginPath()

  // Calculate jaw angle based on time for chomping animation
  const chompSpeed = 150
  const jawAngle = (Math.sin(performance.now() / chompSpeed) + 1) * 0.25 * Math.PI

  let baseRotation = 0
  switch(state.pacman.direction) {
    case 'Up': baseRotation = -Math.PI / 2; break;
    case 'Down': baseRotation = Math.PI / 2; break;
    case 'Left': baseRotation = Math.PI; break;
    case 'Right': baseRotation = 0; break;
  }

  // Draw pacman wedge
  ctx.value.moveTo(px + TILE_SIZE / 2, py + TILE_SIZE / 2)
  ctx.value.arc(
    px + TILE_SIZE / 2,
    py + TILE_SIZE / 2,
    TILE_SIZE / 1.5,
    baseRotation + jawAngle,
    baseRotation + 2 * Math.PI - jawAngle
  )
  ctx.value.fill()

  // 4. Draw Ghosts
  for (const ghost of state.ghosts) {
    const gx = ghost.position.x * TILE_SIZE
    const gy = ghost.position.y * TILE_SIZE

    // Ghost colors
    let color = '#FFF'
    if (ghost.mode === 'Frightened') {
      color = Math.floor(performance.now() / 250) % 2 === 0 ? '#0000FF' : '#FFFFFF'
    } else if (ghost.mode === 'Eaten') {
      color = 'rgba(255,255,255,0.2)' // Mostly transparent
    } else {
      switch(ghost.ghost_type) {
        case 'Blinky': color = '#FF0000'; break;
        case 'Pinky': color = '#FFB8FF'; break;
        case 'Inky': color = '#00FFFF'; break;
        case 'Clyde': color = '#FFB852'; break;
      }
    }

    ctx.value.fillStyle = color
    ctx.value.beginPath()

    // Draw ghost body (rounded top, flat bottom with skirt)
    const radius = TILE_SIZE / 1.5
    const centerX = gx + TILE_SIZE / 2
    const topY = gy + TILE_SIZE / 2 - radius

    ctx.value.arc(centerX, topY + radius, radius, Math.PI, 0)
    ctx.value.lineTo(centerX + radius, gy + TILE_SIZE)

    // Skirt waves
    for (let i = 1; i <= 3; i++) {
        ctx.value.lineTo(centerX + radius - (i * radius * 0.66), gy + TILE_SIZE - (i % 2 === 1 ? 4 : 0))
    }

    ctx.value.lineTo(centerX - radius, topY + radius)
    ctx.value.fill()

    // Draw eyes if not frightened
    if (ghost.mode !== 'Frightened') {
        ctx.value.fillStyle = 'white'
        ctx.value.beginPath()
        ctx.value.arc(centerX - 4, topY + radius - 2, 3, 0, Math.PI * 2)
        ctx.value.arc(centerX + 4, topY + radius - 2, 3, 0, Math.PI * 2)
        ctx.value.fill()

        // Pupils based on direction
        let pxOff = 0, pyOff = 0
        switch(ghost.direction) {
            case 'Up': pyOff = -1; break;
            case 'Down': pyOff = 1; break;
            case 'Left': pxOff = -1; break;
            case 'Right': pxOff = 1; break;
        }
        ctx.value.fillStyle = 'blue'
        ctx.value.beginPath()
        ctx.value.arc(centerX - 4 + pxOff, topY + radius - 2 + pyOff, 1.5, 0, Math.PI * 2)
        ctx.value.arc(centerX + 4 + pxOff, topY + radius - 2 + pyOff, 1.5, 0, Math.PI * 2)
        ctx.value.fill()
    } else {
        // Frightened face features
        ctx.value.fillStyle = '#FFB8AE'
        ctx.value.beginPath()
        ctx.value.arc(centerX - 4, topY + radius - 2, 1.5, 0, Math.PI * 2)
        ctx.value.arc(centerX + 4, topY + radius - 2, 1.5, 0, Math.PI * 2)
        ctx.value.fill()

        ctx.value.strokeStyle = '#FFB8AE'
        ctx.value.lineWidth = 1.5
        ctx.value.beginPath()
        ctx.value.moveTo(centerX - 6, topY + radius + 4)
        ctx.value.lineTo(centerX - 3, topY + radius + 1)
        ctx.value.lineTo(centerX, topY + radius + 4)
        ctx.value.lineTo(centerX + 3, topY + radius + 1)
        ctx.value.lineTo(centerX + 6, topY + radius + 4)
        ctx.value.stroke()
    }
  }

  // Draw "READY!" or "GAME OVER!" text
  if (state.phase === 'Ready' || state.phase === 'GameOver') {
    ctx.value.fillStyle = state.phase === 'Ready' ? '#FFFF00' : '#FF0000'
    ctx.value.font = '20px "Press Start 2P"'
    ctx.value.textAlign = 'center'
    const text = state.phase === 'Ready' ? 'READY!' : 'GAME OVER'
    ctx.value.fillText(text, boardWidth / 2, boardHeight / 2 + 50)
  }

  ctx.value.restore()
}
</script>

<style scoped>
.game-board-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  width: 100vw;
  background-color: #000;
  overflow: hidden;
  position: relative;
}

.hud {
  position: absolute;
  top: 20px;
  width: 100%;
  max-width: 600px;
  display: flex;
  justify-content: space-between;
  padding: 0 20px;
  color: white;
  font-family: 'Press Start 2P', cursive;
  margin-bottom: 0;
  font-size: 16px;
  z-index: 10;
  pointer-events: none;
}

canvas {
  width: 100vw;
  height: 100vh;
  object-fit: contain;
  background-color: #000;
}
</style>
