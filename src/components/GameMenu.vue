<template>
  <div class="game-menu" ref="menuContainer">
    <div class="menu-title" ref="menuTitleRef">PAC-MAN</div>
    <ul>
      <li v-for="(item, index) in menuItems" :key="item.id" :ref="el => { if (el) menuItemRefs[index] = el }"
        @click="item.action" @mouseenter="selectedIndex = index" :class="{ selected: selectedIndex === index }">
        <span class="arrow" v-if="selectedIndex === index">></span> {{ item.label }}
      </li>
    </ul>
    <div class="high-score" ref="highScoreRef">HIGH SCORE: {{ highScore }}</div>
  </div>
</template>

<script setup>
import { ref, onMounted, nextTick } from 'vue';
import { animate } from 'animejs';

const emit = defineEmits(['startGame']);

const menuContainer = ref(null);
const menuTitleRef = ref(null);
const menuItemRefs = ref([]);
const highScoreRef = ref(null);

const highScore = ref(50000); // Example
const selectedIndex = ref(0);

const menuItems = ref([
  { id: 'start', label: 'START GAME', action: () => emit('startGame') },
  { id: 'options', label: 'OPTIONS', action: () => console.log('Options') },
  // { id: 'highscores', label: 'HIGH SCORES', action: () => console.log('High Scores') },
]);

// Keyboard navigation (basic)
onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});
const handleKeydown = (e) => {
  if (!menuContainer.value || !menuContainer.value.offsetParent) return; // Only if menu is visible

  if (e.key === 'ArrowDown') {
    selectedIndex.value = (selectedIndex.value + 1) % menuItems.value.length;
  } else if (e.key === 'ArrowUp') {
    selectedIndex.value = (selectedIndex.value - 1 + menuItems.value.length) % menuItems.value.length;
  } else if (e.key === 'Enter' || e.key === ' ') {
    menuItems.value[selectedIndex.value]?.action();
  }
};

const animateIn = async () => {
  if (!menuContainer.value) return;

  // Ensure refs are populated
  await nextTick();

  const tl = anime.timeline({
    easing: 'easeOutExpo',
    duration: 750,
  });

  tl.add({
    targets: menuTitleRef.value,
    opacity: [0, 1],
    translateY: [-20, 0],
    delay: 300, // Wait for zoom to mostly finish
  })
    .add({
      targets: menuItemRefs.value,
      opacity: [0, 1],
      translateX: [-20, 0],
      delay: anime.stagger(150, { start: 200 }),
    }, '-=300') // Overlap slightly with title animation
    .add({
      targets: highScoreRef.value,
      opacity: [0, 1],
      translateY: [20, 0],
    }, '-=500'); // Overlap

  await tl.finished;
};

defineExpose({ animateIn });
</script>

<style lang="scss" scoped>
@import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

// Gruvbox Colors
$gb-yellow: #fabd2f;
$gb-fg: #ebdbb2;
$gb-red: #fb4934;
$gb-bg-screen: #080818; // Screen background

.game-menu {
  font-family: 'Press Start 2P', cursive;
  color: $gb-fg;
  text-align: center;
  text-transform: uppercase;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  // For initial animation state
  opacity: 1; // Will be set to 0 by animateIn initially if needed
}

.menu-title {
  font-size: 2.5em; // Adjust based on screen size
  color: $gb-yellow;
  margin-bottom: 1.5em;
  text-shadow: 3px 3px 0px $gb-red; // Classic shadow
  opacity: 0; // For animateIn
}

ul {
  list-style: none;
  padding: 0;
  margin: 0 0 1.5em 0;
  width: 80%;
}

li {
  font-size: 1.2em; // Adjust
  margin-bottom: 0.8em;
  cursor: pointer;
  opacity: 0; // For animateIn
  position: relative;
  padding: 5px 0;

  &:hover,
  &.selected {
    color: $gb-yellow;
  }

  .arrow {
    position: absolute;
    left: -25px; // Adjust as needed
    color: $gb-red;
  }
}

.high-score {
  font-size: 1em;
  color: $gb-fg;
  opacity: 0; // For animateIn
}
</style>
