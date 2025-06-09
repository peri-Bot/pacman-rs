// LandingPage.vue
<template>
  <div class="landing-page-container" ref="landingContainer">
    <div class="arcade-wrapper" ref="arcadeWrapperRef">
      <ArcadeMachine ref="arcadeMachineRef">
        <!-- <template #screenContent> -->
        <!--   <ArcadeScreen ref="arcadeScreenRef"> -->
        <!--     <GameMenu v-if="showMenu" ref="gameMenuRef" @startGame="onStartGame" /> -->
        <!--     <!-- Initially, you might show a demo GIF or animation here --> -->
        <!--     <!-- <img v-if="!showMenu && !animationInProgress" src="/img/pacman-demo-loop.gif" alt="Pacman Demo" -->
        -->
        <!--     <!--   class="demo-loop" /> --> -->
        <!--   </ArcadeScreen> -->
        <!-- </template> -->
      </ArcadeMachine>
    </div>

    <!-- Optional: A button to trigger the zoom, or it can be automatic -->
    <button v-if="!animationInProgress && !menuVisible" @click="startZoomAnimation" class="start-experience-button">
      INSERT COIN
    </button>
  </div>
</template>

<script setup>
import { ref, onMounted, nextTick } from 'vue';
import { animate } from 'animejs';
import ArcadeMachine from './../components/ArcadeMachine.vue';
import ArcadeScreen from './../components/ArcadeScreen.vue';
import GameMenu from './../components/GameMenu.vue';
import { useRouter } from 'vue-router'; // If you're using Vue Router

const router = useRouter(); // Or your preferred navigation method

const landingContainer = ref(null);
const arcadeWrapperRef = ref(null);
const arcadeMachineRef = ref(null); // Access child component if needed
const arcadeScreenRef = ref(null);
const gameMenuRef = ref(null);

const showMenu = ref(false);
const menuVisible = ref(false); // To control button visibility after menu is up
const animationInProgress = ref(false);

const startZoomAnimation = async () => {
  if (animationInProgress.value || !arcadeWrapperRef.value || !arcadeScreenRef.value?.getScreenElement()) return;
  animationInProgress.value = true;

  // Get screen dimensions and position relative to the viewport
  // This is crucial for a smooth zoom-to-screen effect
  const screenElement = arcadeScreenRef.value.getScreenElement(); // Assume ArcadeScreen exposes its main element
  if (!screenElement) {
    console.error("Arcade screen element not found!");
    animationInProgress.value = false;
    return;
  }

  const screenRect = screenElement.getBoundingClientRect();
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  // Calculate the scale needed for the screen to (mostly) fill the viewport
  // Target a slightly smaller size than full screen for a better effect
  const targetScreenWidth = viewportWidth * 0.8;
  const scaleFactor = targetScreenWidth / screenRect.width;

  // Calculate translations to center the screen
  // The origin for scale is center by default with anime.js for transforms
  // We want to translate the *wrapper* such that the *screen's center* ends up at the *viewport's center*
  const wrapperRect = arcadeWrapperRef.value.getBoundingClientRect();

  // Screen center relative to viewport
  const screenCenterX = screenRect.left + screenRect.width / 2;
  const screenCenterY = screenRect.top + screenRect.height / 2;

  // Viewport center
  const viewportCenterX = viewportWidth / 2;
  const viewportCenterY = viewportHeight / 2;

  // Translation needed for the wrapper to bring screen's center to viewport's center
  // BEFORE scaling is applied to the wrapper.
  // If scaling arcadeWrapperRef directly, the translation math needs to account for that.
  // It's often easier to have an outer container for positioning and an inner one for scaling.
  // For now, let's assume arcadeWrapperRef is what we scale and translate.

  // Target position for the screen's center *after* scaling
  // (translateX and translateY are applied to the arcadeWrapperRef)
  // This needs to be the difference between the viewport center and the
  // arcadeWrapper's center, adjusted for where the screen is within the wrapper.
  const arcadeWrapperCenterX = wrapperRect.left + wrapperRect.width / 2;
  const arcadeWrapperCenterY = wrapperRect.top + wrapperRect.height / 2;

  const screenCenterRelativeToWrapperX = screenCenterX - wrapperRect.left;
  const screenCenterRelativeToWrapperY = screenCenterY - wrapperRect.top;

  // The final X,Y of the wrapper should be such that:
  // finalWrapperX + screenCenterRelativeToWrapperX * scaleFactor = viewportCenterX
  // finalWrapperY + screenCenterRelativeToWrapperY * scaleFactor = viewportCenterY
  // So, finalWrapperX = viewportCenterX - screenCenterRelativeToWrapperX * scaleFactor
  // And translateX = finalWrapperX - wrapperRect.left (initial position)

  // This simplified version targets the wrapper's center to the viewport center,
  // and assumes the screen is somewhat centered in the wrapper.
  // For precise screen centering, the math gets more involved or you might nest wrappers.
  const translateX = viewportCenterX - (wrapperRect.left + wrapperRect.width / 2);
  const translateY = viewportCenterY - (wrapperRect.top + wrapperRect.height / 2) - (viewportHeight * 0.05); // Slight offset from true center


  // Fade out the demo loop if it exists
  const demoLoopEl = document.querySelector('.demo-loop');
  if (demoLoopEl) {
    anime({
      targets: demoLoopEl,
      opacity: 0,
      duration: 300,
      easing: 'easeInQuad',
    });
  }

  await anime({
    targets: arcadeWrapperRef.value,
    translateX: translateX,
    translateY: translateY,
    scale: scaleFactor,
    // For a more 3D effect (ensure parent has `perspective`):
    // rotateX: ['0deg', '10deg'], // Example
    // rotateY: ['0deg', '-5deg'], // Example
    duration: 1500,
    easing: 'easeInOutExpo', // A nice smooth easing
  }).finished;

  showMenu.value = true;
  menuVisible.value = true; // For hiding "INSERT COIN"

  // Ensure GameMenu is rendered before trying to animate its elements
  await nextTick();
  if (gameMenuRef.value) {
    gameMenuRef.value.animateIn(); // Assume GameMenu has an animateIn method
  }
  animationInProgress.value = false;
};

const onStartGame = () => {
  // Navigate to the actual game route/view
  console.log('Start Game Clicked!');
  router.push('/play'); // Example using Vue Router
};

onMounted(() => {
  // You could trigger the zoom automatically after a delay
  // setTimeout(startZoomAnimation, 2000);
});
</script>

<style lang="scss" scoped>
// Gruvbox Dark Palette (example)
$bg-dark: #282828;
$fg-light: #ebdbb2;
$yellow: #fabd2f;
$red: #cc241d;

.landing-page-container {
  width: 100vw;
  height: 100vh;
  background-color: $bg-dark;
  color: $fg-light;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  overflow: hidden; // Important for zoom effect
  position: relative;
  // For 3D perspective if you use rotateX/Y on the arcade machine
  // perspective: 1000px;
}

.arcade-wrapper {
  // This wrapper will be scaled and translated
  // Set initial size/position if needed, or let ArcadeMachine define it
  position: relative; // Or absolute if you want more control over initial placement
  // transform-origin: center center; // Default, but can be explicit
}

.demo-loop {
  width: 100%; // Assuming it's inside ArcadeScreen which controls its aspect ratio
  height: 100%;
  object-fit: cover; // Or contain, depending on the GIF
}

.start-experience-button {
  position: absolute;
  bottom: 10%;
  left: 50%;
  transform: translateX(-50%);
  padding: 15px 30px;
  background-color: $yellow;
  color: $bg-dark;
  border: none;
  font-family: 'Press Start 2P', cursive; // A good retro font
  font-size: 1.2rem;
  cursor: pointer;
  text-transform: uppercase;
  transition: all 0.1s ease-out;

  &:active {
    transform: translate(-50%, 3px);
  }
}
</style>
