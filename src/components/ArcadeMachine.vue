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

.side-panel.left {
  margin-right: 8px;
}

.side-panel.right {
  margin-left: 8px;
}

.side-art-character {
  width: 35px;
  height: 35px;
  margin-bottom: 15px;
  border-radius: 50%;
}

.side-art-character.ghost {
  border-radius: 18px 18px 5px 5px;
  /* Ghost shape */
  position: relative;
}

.side-art-character.ghost::before,
.side-art-character.ghost::after {
  /* Eyes */
  content: '';
  position: absolute;
  background-color: white;
  width: 8px;
  height: 10px;
  border-radius: 40%;
  top: 8px;
}

.side-art-character.ghost::before {
  left: 6px;
}

/* Pupil left */
.side-art-character.ghost::after {
  right: 6px;
}

/* Pupil right */

.side-art-character.ghost.blue {
  background-color: #39f;
}

.side-art-character.ghost.blue::before,
.side-art-character.ghost.blue::after {
  background-image: radial-gradient(circle at 3px 5px, #000 2px, transparent 2.5px);
}

.side-art-character.ghost.red {
  background-color: #f00;
}

.side-art-character.ghost.red::before,
.side-art-character.ghost.red::after {
  background-image: radial-gradient(circle at 3px 5px, #000 2px, transparent 2.5px);
}

.side-art-character.pacman.yellow {
  background-color: #ff0;
  clip-path: polygon(0% 0%, 100% 0%, 100% 50%, 60% 50%, 60% 100%, 0% 100%);
  /* Facing down-left */
}

.side-art-character.pacman.yellow.alt {
  clip-path: polygon(0% 50%, 40% 50%, 40% 0%, 100% 0%, 100% 100%, 40% 100%);
  /* Facing down-right */
}


.instructions-panel,
.monster-points-panel {
  font-size: 0.45em;
  line-height: 1.4;
  text-align: center;
  padding: 5px;
  border: 1px solid #444;
  background-color: #111;
  border-radius: 3px;
  width: 90%;
  margin-top: 10px;
}

.instructions-title,
.monster-points-title {
  font-size: 1.1em;
  color: #ff0;
  margin-bottom: 5px;
}

.instruction-line {
  margin-bottom: 2px;
}

.monster-points-panel .points-row {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 2px;
  font-size: 0.9em;
}

.monster-points-panel .ghost-icon {
  width: 8px;
  height: 9px;
  background-color: #39f;
  /* Blue */
  border-radius: 4px 4px 1px 1px;
  margin: 0 1px;
  display: inline-block;
}


/* Screen */
.screen {
  flex-grow: 1;
  background-color: #000;
  /* Bezel color */
  padding: 5px;
  /* Bezel thickness */
  display: flex;
  justify-content: center;
  align-items: center;
}

.screen-bezel {
  /* Inner border for screen */
  width: 100%;
  height: 100%;
  border: 1px solid #333;
  background-color: #000;
  /* Actual screen background */
}

.screen-content {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 5px;
  box-sizing: border-box;
}

.screen-header {
  display: flex;
  justify-content: space-between;
  font-size: 0.6em;
  padding: 0 2px;
  color: #fff;
}

.arcade1up-logo-screen {
  font-size: 0.7em;
  color: #FFF;
}

.arcade1up-logo-screen .one {
  color: #FF0000;
}

.scores {
  display: flex;
  justify-content: space-between;
  font-size: 0.7em;
  padding: 2px 2px;
  color: #fff;
}

.scores .high {
  color: #FF0000;
}

.game-board {
  flex-grow: 1;
  background-color: #000;
  position: relative;
  margin: 5px 0;
  border: 1px solid #002244;
  /* Dark blue border for maze area */
  overflow: hidden;
  /* Keep elements inside */
}

/* Simplified Maze Elements */
.maze-dot {
  position: absolute;
  width: 3px;
  height: 3px;
  background-color: #fff2cc;
  /* Off-white dots */
  border-radius: 50%;
}

.power-pellet {
  position: absolute;
  width: 8px;
  height: 8px;
  background-color: #fff2cc;
  border-radius: 50%;
  animation: blink 1s infinite alternate;
  box-shadow: 0 0 3px #fff2cc;
}

@keyframes blink {

  0%,
  100% {
    transform: scale(1);
    opacity: 1;
  }

  50% {
    transform: scale(0.8);
    opacity: 0.7;
  }
}

.maze-h-line,
.maze-v-line {
  position: absolute;
  background-color: #0077cc;
  /* Blue maze lines */
  box-shadow: 0 0 2px #3af;
}

.maze-h-line {
  height: 4px;
  border-radius: 2px;
}

.maze-v-line {
  width: 4px;
  border-radius: 2px;
}


.game-pacman-char {
  width: 14px;
  height: 14px;
  background-color: yellow;
  border-radius: 50%;
  position: absolute;
  bottom: 25%;
  /* Example position */
  left: 10%;
  /* Example position */
  clip-path: polygon(0 0, 100% 0, 100% 40%, 60% 50%, 100% 60%, 100% 100%, 0 100%);
  /* Mouth open to right */
}

.game-ghost-char {
  width: 14px;
  height: 16px;
  position: absolute;
  border-radius: 7px 7px 2px 2px;
  /* Ghost shape */
}

.game-ghost-char.blue {
  background-color: #39f;
}

.game-ghost-char.red {
  background-color: #f00;
}

/* Eyes for game ghosts */
.game-ghost-char::before,
.game-ghost-char::after {
  content: '';
  position: absolute;
  width: 5px;
  height: 6px;
  background-color: white;
  border-radius: 50%;
  top: 3px;
}

.game-ghost-char::before {
  left: 1.5px;
}

.game-ghost-char::after {
  right: 1.5px;
}


.game-fruit.cherry {
  width: 10px;
  height: 10px;
  background-color: red;
  border-radius: 50% 50% 50% 0;
  /* Cherry shape */
  position: absolute;
  bottom: 10%;
  left: 70%;
  transform: rotate(-45deg);
}

.game-fruit.cherry::before {
  /* stem */
  content: '';
  width: 1.5px;
  height: 5px;
  background-color: #006400;
  /* DarkGreen */
  position: absolute;
  top: -3px;
  left: 4px;
  transform: rotate(25deg);
}


.lives-display {
  display: flex;
  align-items: center;
  padding: 2px 5px;
  margin-top: auto;
}

.pacman-life {
  width: 12px;
  height: 12px;
  background-color: yellow;
  border-radius: 50%;
  margin-right: 5px;
  clip-path: polygon(0 0, 100% 0, 100% 40%, 60% 50%, 100% 60%, 100% 100%, 0 100%);
}

.fruit-indicator.cherry {
  width: 8px;
  height: 8px;
  background-color: red;
  border-radius: 50% 50% 50% 0;
  margin-left: auto;
  /* Push to the right */
  transform: rotate(-45deg);
}

.fruit-indicator.cherry::before {
  content: '';
  width: 1px;
  height: 3px;
  background-color: #006400;
  position: absolute;
  top: -2px;
  left: 3px;
  transform: rotate(25deg);
}

/* NEW WRAPPER for projecting front yellow parts */
.projecting-front-section {
  transform: translateZ(15px) rotateX(-3deg);
  /* Bring forward and tilt up */
  box-shadow: 0px 6px 15px -2px rgba(0, 0, 0, 0.45);
  /* Shadow to cast on cabinet */
  position: relative;
  z-index: 1;
  /* Ensure it's above the default Z plane, screen is negative */
}

/* Control Panel */
.control-panel {
  background-color: #FFFF00;
  padding: 10px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border: 2px solid #000;
  border-radius: 5px 5px 0 0;
  /* Rounded top corners, flat bottom to meet lower panel */
  margin-bottom: 0;
  /* Removed margin, will be seamless with lower panel */
  min-height: 70px;
}

.control-section {
  display: flex;
}

.control-section.left {
  margin-right: auto;
}

.control-section.right {
  margin-left: auto;
}

.joystick-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 0 10px;
  /* Spacing if sections are close */
}

.joystick-base {
  width: 45px;
  height: 45px;
  background-color: #333;
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: flex-end;
  /* Stick comes from bottom */
  margin-bottom: 2px;
  border: 1.5px solid #111;
  padding-bottom: 2px;
  /* To show stick base */
}

.joystick-stick {
  width: 10px;
  height: 20px;
  /* Part of stick inside base */
  background-color: #111;
  /* Stick color */
  position: relative;
}

.joystick-stick::before {
  /* Ball top */
  content: '';
  width: 22px;
  height: 22px;
  background-color: #FF0000;
  border-radius: 50%;
  position: absolute;
  bottom: 15px;
  /* Sits on top of stick, adjust height */
  left: -6px;
  border: 1px solid #a00000;
  box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.5);
}

.joystick-label {
  font-size: 0.55em;
  color: #000;
  font-weight: bold;
  margin-top: 2px;
}

.control-button {
  width: 28px;
  height: 28px;
  background-color: #fff;
  border: 1.5px solid #000;
  border-radius: 50%;
  margin: 0 5px;
  cursor: pointer;
  box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8em;
  color: black;
  font-weight: bold;
}

.control-button.player-one {
  background-color: #f00;
  color: white;
}

.control-button:active {
  transform: translateY(1px) translateX(0.5px);
  box-shadow: 0.5px 0.5px 1px rgba(0, 0, 0, 0.4);
}

/* Lower Front Panel */
.lower-front-panel {
  background-color: #FFFF00;
  padding: 10px 15px;
  border-left: 2px solid #000;
  border-right: 2px solid #000;
  border-bottom: 2px solid #000;
  border-top: none;
  /* Joins with control panel border */
  border-radius: 0 0 3px 3px;
  /* Only bottom corners rounded */
  display: flex;
  justify-content: space-between;
  align-items: center;
  min-height: 60px;
}

.lower-art {
  flex-grow: 1;
  display: flex;
  align-items: center;
}

.lower-pacman {
  width: 40px;
  height: 40px;
  background-color: #000;
  /* Black Pac-Man on yellow */
  border-radius: 50%;
  position: relative;
  clip-path: polygon(0 50%, 40% 50%, 40% 0, 100% 0, 100% 100%, 40% 100%);
  /* Mouth open left */
}

.lower-pacman::before {
  /* Eye */
  content: '';
  width: 6px;
  height: 8px;
  background-color: red;
  border-radius: 50%;
  position: absolute;
  top: 8px;
  left: 18px;
}

.lower-dots {
  display: flex;
  margin: 0 5px;
}

.lower-dots .dot {
  width: 6px;
  height: 6px;
  background-color: #000;
  border-radius: 50%;
  margin: 0 2px;
}

.lower-ghost {
  width: 30px;
  height: 32px;
  border-radius: 15px 15px 3px 3px;
  /* Ghost shape */
  position: relative;
}

.lower-ghost.blue {
  background-color: #0000FF;
}

/* Eyes for lower ghost */
.lower-ghost::before,
.lower-ghost::after {
  content: '';
  position: absolute;
  width: 8px;
  height: 10px;
  background-color: white;
  border-radius: 3px;
  top: 6px;
}

.lower-ghost::before {
  left: 4px;
}

.lower-ghost::after {
  right: 4px;
}


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
