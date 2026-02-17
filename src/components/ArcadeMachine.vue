<template>
  <div class="arcade-machine">
    <div class="marquee">
      <div class="arcade1up-logo-top">
        ARCADE <span class="one">1</span>UP
        <div class="party-cade-text">PARTY-CADE</div>
      </div>
      <div class="pacman-logo-marquee-container">
        <div class="pacman-art-left">
          <div class="ghost-marquee blue"></div>
        </div>
        <div class="pacman-logo-marquee">PAC-MAN™</div>
        <img src="../assets/pacman-ghost.svg" class="pacman" />
        <div class="pacman-art-right">
          <div class="pacman-char-marquee"></div>
        </div>
      </div>
      <div class="included-games-sticker">
        <span class="number">12</span>
        <span class="text">INCLUDED<br>GAMES</span>
      </div>
    </div>

    <div class="main-cabinet-body">
      <div class="screen-area-wrapper">
        <div class="side-panel left">
          <div class="side-art-character ghost blue"></div>
          <div class="side-art-character pacman yellow"></div>
          <div class="instructions-panel">
            <div class="instructions-title">INSTRUCTIONS</div>
            <div class="instruction-line">MOVE PAC-MAN WITH JOYSTICK</div>
            <div class="instruction-line">EAT ALL DOTS TO COMPLETE SCREEN</div>
            <div class="instruction-line">EAT POWER PELLETS TO EAT GHOSTS</div>
            <div class="instruction-line">EAT FRUIT FOR BONUS POINTS</div>
          </div>
        </div>

        <div class="screen">
          <div class="screen-bezel">
            <div class="screen-content">
              <div class="screen-header">
                <span class="score-label">1UP</span>
                <span class="arcade1up-logo-screen">ARCADE <span class="one">1</span>UP</span>
                <span class="score-label">HIGH SCORE</span>
                <span class="score-label">2UP</span>
              </div>
              <div class="scores">
                <span class="score-value">390</span>
                <span class="score-value high">12920</span>
                <span class="score-value">70</span>
              </div>
              <div class="game-board">
                <!-- Simplified Maze Representation -->
                <div class="maze-dot" style="top: 10%; left: 10%;"></div>
                <div class="maze-dot" style="top: 10%; left: 20%;"></div>
                <div class="maze-dot" style="top: 10%; left: 80%;"></div>
                <div class="maze-dot" style="top: 10%; left: 90%;"></div>

                <div class="power-pellet" style="top: 15%; left: 5%;"></div>
                <div class="power-pellet" style="top: 15%; left: 90%;"></div>
                <div class="power-pellet" style="top: 80%; left: 5%;"></div>
                <div class="power-pellet" style="top: 80%; left: 90%;"></div>

                <!-- Horizontal Lines -->
                <div class="maze-h-line" style="top: 20%; left: 10%; width: 80%;"></div>
                <div class="maze-h-line" style="top: 30%; left: 20%; width: 20%;"></div>
                <div class="maze-h-line" style="top: 30%; left: 60%; width: 20%;"></div>
                <div class="maze-h-line" style="top: 40%; left: 0%; width: 100%;"></div>
                <div class="maze-h-line" style="top: 50%; left: 30%; width: 40%;"></div>
                <div class="maze-h-line" style="top: 60%; left: 0%; width: 30%;"></div>
                <div class="maze-h-line" style="top: 60%; left: 70%; width: 30%;"></div>
                <div class="maze-h-line" style="top: 70%; left: 10%; width: 80%;"></div>

                <!-- Vertical Lines -->
                <div class="maze-v-line" style="top: 5%; left: 25%; height: 20%;"></div>
                <div class="maze-v-line" style="top: 5%; left: 75%; height: 20%;"></div>
                <div class="maze-v-line" style="top: 40%; left: 15%; height: 25%;"></div>
                <div class="maze-v-line" style="top: 40%; left: 85%; height: 25%;"></div>
                <div class="maze-v-line" style="top: 70%; left: 40%; height: 20%;"></div>
                <div class="maze-v-line" style="top: 70%; left: 60%; height: 20%;"></div>


                <div class="game-pacman-char"></div>
                <div class="game-ghost-char blue" style="top: 45%; left: 25%;"></div>
                <div class="game-ghost-char red" style="top: 25%; left: 48%;"></div> <!-- Representative position -->
                <div class="game-fruit cherry"></div>
              </div>
              <div class="lives-display">
                <div class="pacman-life"></div>
                <div class="pacman-life"></div>
                <div class="fruit-indicator cherry"></div>
              </div>
            </div>
          </div>
        </div>

        <div class="side-panel right">
          <div class="side-art-character ghost red"></div>
          <div class="side-art-character pacman yellow alt"></div>
          <div class="monster-points-panel">
            <div class="monster-points-title">MONSTER<br>POINTS</div>
            <div class="points-row">
              <div class="ghost-icon blue"></div> = 200
            </div>
            <div class="points-row">
              <div class="ghost-icon blue"></div>
              <div class="ghost-icon blue"></div> = 400
            </div>
            <div class="points-row">
              <div class="ghost-icon blue"></div>
              <div class="ghost-icon blue"></div>
              <div class="ghost-icon blue"></div> = 800
            </div>
            <div class="points-row">
              <div class="ghost-icon blue"></div>
              <div class="ghost-icon blue"></div>
              <div class="ghost-icon blue"></div>
              <div class="ghost-icon blue"></div> = 1600
            </div>
          </div>
        </div>
      </div>

      <div class="projecting-front-section">
        <div class="control-panel">
          <div class="control-section left">
            <button class="control-button white"></button>
            <button class="control-button white player-one">1</button>
          </div>
          <div class="joystick-area">
            <div class="joystick-base">
              <div class="joystick-stick"></div>
            </div>
            <div class="joystick-label">MOVE</div>
          </div>
          <div class="control-section right">
            <button class="control-button white"></button>
            <button class="control-button white"></button>
          </div>
        </div>

        <div class="lower-front-panel">
          <div class="lower-art">
            <div class="lower-pacman"></div>
            <div class="lower-dots">
              <div class="dot"></div>
              <div class="dot"></div>
              <div class="dot"></div>
            </div>
            <div class="lower-ghost blue"></div>
          </div>
          <div class="speaker-grill">
            <div class="hole" v-for="n in 9" :key="`hole-${n}`"></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PacmanArcadeMachine',
};
</script>

<style scoped>
/* Import Google Font for pixelated look - Ensure this is in your global styles or HTML head */
/* @import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap'); */
.pacman {
  color: white;
}

.arcade-machine {
  width: 380px;
  background-color: #111;
  border: 3px solid #000;
  box-shadow: 0 0 20px rgba(0, 0, 0, 0.7);
  display: flex;
  flex-direction: column;
  font-family: 'Press Start 2P', 'Arial', sans-serif;
  /* Fallback font */
  color: white;
  margin: 20px auto;
  border-radius: 10px 10px 5px 5px;
  perspective: 900px;
  /* ADDED for 3D effect */
}

/* Marquee */
.marquee {
  background-color: #000;
  padding: 10px 15px;
  text-align: center;
  border-bottom: 5px solid #222;
  position: relative;
  min-height: 100px;
  /* Ensure space for elements */
  z-index: 2;
  /* Ensure marquee is visually on top of potentially transformed body parts */
}

.arcade1up-logo-top {
  font-size: 1.4em;
  color: #FFF;
  font-weight: bold;
  letter-spacing: 0.05em;
  line-height: 1;
}

.arcade1up-logo-top .one {
  color: #FF0000;
  font-style: italic;
  font-size: 1.1em;
}

.party-cade-text {
  font-size: 0.5em;
  color: #bbb;
  margin-top: 0px;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.pacman-logo-marquee-container {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 8px;
}

.pacman-art-left,
.pacman-art-right {
  display: flex;
  align-items: center;
  margin: 0 5px;
}

.ghost-marquee {
  width: 28px;
  height: 30px;
  background-color: #39f;
  /* Blue ghost */
  border-radius: 15px 15px 4px 4px;
  position: relative;
}

.ghost-marquee::before,
.ghost-marquee::after {
  /* Eyes */
  content: '';
  position: absolute;
  background-color: white;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  top: 8px;
}

.ghost-marquee::before {
  left: 5px;
}

.ghost-marquee::after {
  right: 5px;
}

.pacman-char-marquee {
  width: 30px;
  height: 30px;
  background-color: #ff0;
  /* Yellow Pac-Man */
  border-radius: 50%;
  clip-path: polygon(0% 0%, 50% 0%, 50% 40%, 100% 50%, 50% 60%, 50% 100%, 0% 100%);
  /* Mouth open right */
}


.pacman-logo-marquee {
  font-size: 2.5em;
  /* Adjusted for container */
  font-weight: bold;
  color: #FFFF00;
  text-shadow:
    3px 3px 0 #FF0000,
    /* Red shadow */
    -2px -2px 0 #39f;
  /* Blue highlight like */
  line-height: 1;
  margin: 0 5px;
}

.included-games-sticker {
  position: absolute;
  top: 8px;
  right: 8px;
  background-color: #5050ff;
  color: white;
  padding: 5px 7px;
  border-radius: 4px;
  transform: rotate(10deg);
  border: 1.5px solid white;
  text-align: center;
  line-height: 0.8;
  box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.4);
}

.included-games-sticker .number {
  font-size: 1.1em;
  display: block;
  font-weight: bold;
}

.included-games-sticker .text {
  font-size: 0.45em;
  display: block;
  letter-spacing: 0.05em;
}

/* Main Cabinet Body */
.main-cabinet-body {
  padding: 10px;
  background-color: #202020;
  /* Dark gray panel */
  flex-grow: 1;
  transform-style: preserve-3d;
  /* ADDED for 3D effect */
  position: relative;
  /* For stacking context of 3D children */
}

.screen-area-wrapper {
  display: flex;
  background-color: #000;
  padding: 10px;
  margin-bottom: 10px;
  border-radius: 5px;
  /* ADDED/MODIFIED for 3D effect */
  transform: translateZ(-20px) rotateX(3deg);
  box-shadow: 0px 2px 10px rgba(0, 0, 0, 0.6) inset;
  /* Inset shadow for depth */
}

.side-panel {
  width: 65px;
  /* Approx width */
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 5px 0;
}

???LINES MISSING
???LINES MISSING
???LINES MISSING
.speaker-grill {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 3px;
  width: 30px;
  margin-left: auto;
  /* Push to right if art doesn't fill space */
}

.speaker-grill .hole {
  width: 5px;
  height: 5px;
  background-color: #000;
  border-radius: 50%;
}
</style>
