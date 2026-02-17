<template>
  <div class="text-center content-center justify-center ">
    <div class="pacman-logo-marquee">PAC-MAN</div>
    <div class="flex flex-row justify-center py-6">
      <div class="p-3">
        <svg class="text-red-500 fill-current h-18 w-20" xmlns="http://www.w3.org/2000/svg" xml:space="preserve"
          width="800" height="800" viewBox="0 0 333.561 333.561">
          <path
            d="M295.023 70.021c-10.653-20.379-25.378-36.617-43.763-48.265C228.47 7.32 200.047 0 166.78 0S105.09 7.32 82.3 21.757C63.914 33.404 49.19 49.643 38.537 70.022 20.792 103.966 20.28 137.91 20.28 141.668v182.064c0 1.4.001 2.673.036 3.596.214 5.784 4.052 6.233 5.223 6.233 2.327 0 3.61-1.173 7.316-4.88l.213-.214 34.969-44.641 34.895 44.274.392.441c2.747 2.745 7.7 4.738 11.779 4.738h3.416c4.08 0 9.033-1.993 11.779-4.739l.21-.21 35.064-44.56 34.909 44.311.393.441c2.756 2.756 7.71 4.756 11.779 4.756h3.416c4.08 0 9.034-1.993 11.78-4.739l.21-.21 35.063-44.559 34.906 44.444.396.447c1.47 1.47 5.23 4.888 8.402 4.888 3.98 0 6.452-3.763 6.452-9.82V141.668c.002-3.759-.51-37.702-18.255-71.647zM109.354 170.146c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59zm114 0c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59z" />
        </svg>
      </div>

      <div class="p-3">
        <svg class="text-yellow-500 fill-current h-18  w-20" xmlns="http://www.w3.org/2000/svg" xml:space="preserve"
          width="800" height="800" viewBox="0 0 333.561 333.561">
          <path
            d="M295.023 70.021c-10.653-20.379-25.378-36.617-43.763-48.265C228.47 7.32 200.047 0 166.78 0S105.09 7.32 82.3 21.757C63.914 33.404 49.19 49.643 38.537 70.022 20.792 103.966 20.28 137.91 20.28 141.668v182.064c0 1.4.001 2.673.036 3.596.214 5.784 4.052 6.233 5.223 6.233 2.327 0 3.61-1.173 7.316-4.88l.213-.214 34.969-44.641 34.895 44.274.392.441c2.747 2.745 7.7 4.738 11.779 4.738h3.416c4.08 0 9.033-1.993 11.779-4.739l.21-.21 35.064-44.56 34.909 44.311.393.441c2.756 2.756 7.71 4.756 11.779 4.756h3.416c4.08 0 9.034-1.993 11.78-4.739l.21-.21 35.063-44.559 34.906 44.444.396.447c1.47 1.47 5.23 4.888 8.402 4.888 3.98 0 6.452-3.763 6.452-9.82V141.668c.002-3.759-.51-37.702-18.255-71.647zM109.354 170.146c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59zm114 0c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59z" />
        </svg>
      </div>
      <div class="p-3">
        <svg class="text-pink-500 fill-current h-18 w-20" xmlns="http://www.w3.org/2000/svg" xml:space="preserve"
          width="800" height="800" viewBox="0 0 333.561 333.561">
          <path
            d="M295.023 70.021c-10.653-20.379-25.378-36.617-43.763-48.265C228.47 7.32 200.047 0 166.78 0S105.09 7.32 82.3 21.757C63.914 33.404 49.19 49.643 38.537 70.022 20.792 103.966 20.28 137.91 20.28 141.668v182.064c0 1.4.001 2.673.036 3.596.214 5.784 4.052 6.233 5.223 6.233 2.327 0 3.61-1.173 7.316-4.88l.213-.214 34.969-44.641 34.895 44.274.392.441c2.747 2.745 7.7 4.738 11.779 4.738h3.416c4.08 0 9.033-1.993 11.779-4.739l.21-.21 35.064-44.56 34.909 44.311.393.441c2.756 2.756 7.71 4.756 11.779 4.756h3.416c4.08 0 9.034-1.993 11.78-4.739l.21-.21 35.063-44.559 34.906 44.444.396.447c1.47 1.47 5.23 4.888 8.402 4.888 3.98 0 6.452-3.763 6.452-9.82V141.668c.002-3.759-.51-37.702-18.255-71.647zM109.354 170.146c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59zm114 0c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59z" />
        </svg>
      </div>
      <div class="p-3">
        <svg class="text-blue-500 fill-current h-18 w-20" xmlns="http://www.w3.org/2000/svg" xml:space="preserve"
          width="800" height="800" viewBox="0 0 333.561 333.561">
          <path
            d="M295.023 70.021c-10.653-20.379-25.378-36.617-43.763-48.265C228.47 7.32 200.047 0 166.78 0S105.09 7.32 82.3 21.757C63.914 33.404 49.19 49.643 38.537 70.022 20.792 103.966 20.28 137.91 20.28 141.668v182.064c0 1.4.001 2.673.036 3.596.214 5.784 4.052 6.233 5.223 6.233 2.327 0 3.61-1.173 7.316-4.88l.213-.214 34.969-44.641 34.895 44.274.392.441c2.747 2.745 7.7 4.738 11.779 4.738h3.416c4.08 0 9.033-1.993 11.779-4.739l.21-.21 35.064-44.56 34.909 44.311.393.441c2.756 2.756 7.71 4.756 11.779 4.756h3.416c4.08 0 9.034-1.993 11.78-4.739l.21-.21 35.063-44.559 34.906 44.444.396.447c1.47 1.47 5.23 4.888 8.402 4.888 3.98 0 6.452-3.763 6.452-9.82V141.668c.002-3.759-.51-37.702-18.255-71.647zM109.354 170.146c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59zm114 0c-17.999 0-32.59-14.591-32.59-32.59s14.591-32.59 32.59-32.59 32.591 14.591 32.591 32.59-14.592 32.59-32.591 32.59z" />
        </svg>
      </div>
      <div class="px-30 p-3">
        <svg class="text-yellow-400 fill-current h-20 w-20" xmlns="http://www.w3.org/2000/svg" xml:space="preserve"
          id="Icons" version="1.1" viewBox="0 0 32 32">
          <path
            d="m19.4 16 7.4-7.4c.4-.4.4-.9.1-1.3C24.2 3.9 20.3 2 16 2 8.3 2 2 8.3 2 16s6.3 14 14 14c4.3 0 8.2-1.9 10.9-5.2.3-.4.3-1-.1-1.3L19.4 16zM14 14c-1.7 0-3-1.3-3-3s1.3-3 3-3 3 1.3 3 3-1.3 3-3 3z" />
        </svg>
      </div>
    </div>
    <div class=" flex items-center justify-center">
      <!-- Bar track -->
      <div class="w-full max-w-md mx-auto mt-6 bg-gray-800 rounded-full h-4 overflow-hidden shadow-lg">
        <!-- Bar fill -->
        <div class="bg-yellow-400 h-full transition-all duration-300 ease-in-out"
          :style="{ width: `${loadingProgress}%` }"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue' // Import onUnmounted
import { useRouter } from 'vue-router' // Import useRouter

const loadingProgress = ref(0)
const router = useRouter() // Get router instance
let intervalId = null; // To store the interval ID

onMounted(() => {
  intervalId = setInterval(() => {
    loadingProgress.value += 1
    if (loadingProgress.value >= 100) {
      loadingProgress.value = 100; // Ensure it's exactly 100 for display
      clearInterval(intervalId)
      intervalId = null; // Reset intervalId after clearing

      // Redirect to ArcadeMachine.vue
      // Make sure you have a route named 'ArcadeMachine' or change to path: '/path-to-arcade'
      router.push({ name: 'ArcadeMachine' })
    }
  }, 50) // Adjust time for faster/slower loading (50ms * 100 = 5 seconds)
})

// Clean up the interval if the component is unmounted before loading finishes
onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
})
</script>


<style scoped>
.pacman-logo-marquee {
  font-size: 4.0em;
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
</style>
