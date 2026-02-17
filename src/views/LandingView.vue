<template>
  <div>
    <!-- Other content for the landing page, if any, can go above or below -->
    <LoadingScreen />
    <!-- LoadingScreen component will handle its own display and redirect -->
  </div>
</template>

<script setup>
import LoadingScreen from '../components/LoadingScreen.vue'; // Make sure path is correct
</script>

<style scoped>
/* Styles for LandingView */
</style>
