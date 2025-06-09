<template>
  <div class="arcade-screen" ref="screenElementRef">
    <div class="scanlines"></div> <!-- Optional scanline effect -->
    <div class="screen-glare"></div> <!-- Optional glare effect -->
    <div class="screen-content-wrapper">
      <slot></slot> <!-- GameMenu or demo loop will go here -->
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
const screenElementRef = ref(null);

// Method to allow parent to get the screen element for calculations
const getScreenElement = () => {
  return screenElementRef.value;
};

defineExpose({ getScreenElement });
</script>

<style lang="scss" scoped>
.arcade-screen {
  width: 100%;
  // For a 4:3 aspect ratio, if width is 100%, padding-bottom is 75%
  padding-bottom: 75%; // This creates a 4:3 aspect ratio box
  position: relative;
  background-color: #080818; // Dark blue/black for CRT feel
  border-radius: 10px; // CRT curvature
  overflow: hidden; // Crucial for effects and content
  box-shadow: inset 0 0 20px rgba(0, 0, 0, 0.7); // Inner shadow for depth

  // Slight bulge effect for CRT (optional, can be complex)
  // transform: perspective(500px) rotateX(1deg) rotateY(-1deg) scale(1.02);
}

.screen-content-wrapper {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 5%; // Inner padding for screen content
}

// Optional Scanlines Effect
.scanlines {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 1;
  background: linear-gradient(to bottom,
      rgba(20, 20, 40, 0) 50%,
      rgba(0, 0, 0, 0.25) 51% // Darker line
    );
  background-size: 100% 4px; // Adjust line thickness and spacing
  animation: scanline-anim 10s linear infinite;
}

@keyframes scanline-anim {
  0% {
    background-position: 0 0;
  }

  100% {
    background-position: 0 -20px;
  }

  // Slower or faster scroll
}

// Optional Glare Effect
.screen-glare {
  position: absolute;
  top: 5%;
  left: 5%;
  width: 50%;
  height: 70%;
  pointer-events: none;
  z-index: 2;
  background: radial-gradient(circle at 30% 30%,
      rgba(220, 220, 255, 0.15) 0%, // Lighter glare center
      rgba(220, 220, 255, 0.0) 50% // Fades out
    );
  transform: skewX(-15deg) rotate(-5deg);
  opacity: 0.7;
}
</style>
