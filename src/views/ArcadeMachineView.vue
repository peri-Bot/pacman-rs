<script setup lang="ts">
import { TresCanvas, useTresContext, useRaycaster } from '@tresjs/core'
import { OrbitControls } from '@tresjs/cientos'
import { SRGBColorSpace, NoToneMapping, Vector3, CanvasTexture, LinearFilter } from 'three'
import { onMounted, ref, watch } from 'vue'
import { gsap } from 'gsap'
import html2canvas from 'html2canvas'

// NEW: Import the menu component
import ArcadeMenu from '../components/GameMenu.vue'

// --- Refs and Context ---
const controlsRef = ref()
const screenGroupRef = ref()
const screenMeshRef = ref() // NEW: Ref for the screen mesh itself
const menuComponentRef = ref() // NEW: Ref for the hidden menu component
const menuTexture = ref<CanvasTexture | null>(null) // NEW: Ref for our dynamic texture

// --- Raycasting for Interaction ---
const { raycaster, pointer } = useRaycaster()

const handleCanvasClick = () => {
  if (!screenMeshRef.value || !menuComponentRef.value) return

  // Find intersects
  const intersects = raycaster.intersectObject(screenMeshRef.value)

  if (intersects.length > 0) {
    const firstIntersection = intersects[0]
    // The UV coordinate tells us where on the texture the click happened (from 0.0 to 1.0)
    const uv = firstIntersection.uv
    if (!uv) return

    // Get the dimensions of the hidden menu component
    const menuEl = menuComponentRef.value.$el
    const rect = menuEl.getBoundingClientRect()

    // Translate UV coordinates to pixel coordinates
    const x = rect.left + uv.x * rect.width
    const y = rect.top + uv.y * rect.height

    // Find the element at that coordinate and simulate a click
    const element = document.elementFromPoint(x, y)
    if (element) {
      ;(element as HTMLElement).click()
    }
  }
}

// --- Texture Generation ---
const updateMenuTexture = async () => {
  if (menuComponentRef.value) {
    const menuEl = menuComponentRef.value.$el
    const canvas = await html2canvas(menuEl, {
      backgroundColor: null, // Use component's background
    })

    if (!menuTexture.value) {
      // Create the texture for the first time
      menuTexture.value = new CanvasTexture(canvas)
      menuTexture.value.minFilter = LinearFilter // Prevents blurriness
      menuTexture.value.needsUpdate = true
    } else {
      // Update existing texture
      menuTexture.value.image = canvas
      menuTexture.value.needsUpdate = true
    }
  }
}

// --- Animation and Setup ---
onMounted(() => {
  const { camera } = useTresContext()

  // Initial texture generation
  updateMenuTexture()

  // The rest of your zoom animation logic
  setTimeout(() => {
    if (!controlsRef.value || !camera.value || !screenGroupRef.value) return

    const controls = controlsRef.value.value
    const screenGroup = screenGroupRef.value
    const endTarget = new Vector3()
    screenGroup.getWorldPosition(endTarget)
    const endPosition = new Vector3(endTarget.x, endTarget.y, endTarget.z + 1.52)
    controls.enabled = false
    const tl = gsap.timeline({
      onComplete: () => {
        controls.enabled = false
        controls.target.copy(endTarget)
      },
    })
    tl.to(camera.value.position, {
      duration: 2.5,
      x: endPosition.x,
      y: endPosition.y,
      z: endPosition.z,
      ease: 'power3.inOut',
    })
    tl.to(
      controls.target,
      {
        duration: 2.5,
        x: endTarget.x,
        y: endTarget.y,
        z: endTarget.z,
        ease: 'power3.inOut',
      },
      '<',
    )
  }, 100)
})

// --- (No changes to gl, dimensions, or initial camera settings) ---
const gl = {
  clearColor: '#1A1A1A',
  shadows: true,
  alpha: false,
  outputColorSpace: SRGBColorSpace,
  toneMapping: NoToneMapping,
}
const mainBodyWidth = 2.0
const mainBodyHeight = 2.8
const mainBodyDepth = 1.0
const controlPanelBaseWidth = 2
const controlPanelBaseHeight = 1.2
const controlPanelBaseDepth = 1.3
const cpBaseMeshY = controlPanelBaseHeight / 2
const mainBodyMeshY = controlPanelBaseHeight + mainBodyHeight / 2
const mainBodyMeshZ = controlPanelBaseDepth - mainBodyDepth / 2
const screenWidth = mainBodyWidth * 0.8
const screenHeight = mainBodyHeight * 0.45
const screenBezelThickness = 0.05
const screenRelToMainBodyCenterY = mainBodyHeight * 0.05
const screenGroupY = mainBodyMeshY + screenRelToMainBodyCenterY
const screenGroupZ = mainBodyMeshZ - mainBodyDepth / 2 + 0.01
const marqueeH = 0.35
const marqueeD = 0.25
const marqueeGroupY = mainBodyMeshY + mainBodyHeight / 2 - marqueeH / 2 + 0.05
const marqueeGroupZ = mainBodyMeshZ - mainBodyDepth / 2 + marqueeD / 2
const controlsSurfaceY = controlPanelBaseHeight + 0.01
const controlsGroupZ = controlPanelBaseDepth * 0.25
const cameraLookAtY = controlPanelBaseHeight * 0.6 + mainBodyHeight * 0.2
const cameraPosition = new Vector3(2.818696123909137, 3.7651027924537264, 7.903299509850072)
const cameraLookAtTarget = new Vector3(0, cameraLookAtY, mainBodyMeshZ * 0.3)
</script>

<template>
  <!-- NEW: Wrapper for canvas and hidden UI -->
  <div class="scene-container">
    <!-- The hidden menu that we will render to the texture -->
    <div class="ui-overlay">
      <ArcadeMenu ref="menuComponentRef" />
    </div>

    <!-- Add the click handler to the canvas -->
    <TresCanvas v-bind="gl" window-size @click="handleCanvasClick">
      <TresPerspectiveCamera :position="cameraPosition" :look-at="cameraLookAtTarget" :fov="45" />
      <OrbitControls ref="controlsRef" :target="cameraLookAtTarget" />

      <TresGroup>
        <!-- ... (All other arcade machine parts are unchanged) ... -->
        <TresMesh :position="[0, cpBaseMeshY, -0.05]" cast-shadow receive-shadow>
          <TresBoxGeometry
            :args="[controlPanelBaseWidth, controlPanelBaseHeight, controlPanelBaseDepth]"
          />
          <TresMeshStandardMaterial />
        </TresMesh>
        <TresMesh :position="[0, mainBodyMeshY, -0.19999999999999996]" cast-shadow receive-shadow>
          <TresBoxGeometry :args="[mainBodyWidth, 3, mainBodyDepth]" />
          <TresMeshToonMaterial color="#4A4A4A" />
        </TresMesh>

        <TresGroup ref="screenGroupRef" :position="[0, screenGroupY, screenGroupZ]">
          <TresMesh :position-z="-screenBezelThickness / 2 - 0.01" cast-shadow>
            <TresBoxGeometry
              :args="[
                screenWidth + screenBezelThickness * 2,
                screenHeight + screenBezelThickness * 2,
                screenBezelThickness,
              ]"
            />
            <TresMeshToonMaterial color="#282828" />
          </TresMesh>
          <!-- MODIFIED: The Screen Mesh -->
          <TresMesh ref="screenMeshRef">
            <TresPlaneGeometry :args="[screenWidth, screenHeight]" />
            <!-- Apply the dynamic texture to the material's map -->
            <TresMeshStandardMaterial
              v-if="menuTexture"
              :map="menuTexture"
              :emissive-map="menuTexture"
              emissive="#ffffff"
              :emissive-intensity="0.7"
            />
          </TresMesh>
        </TresGroup>

        <!-- ... (Rest of the arcade machine parts are unchanged) ... -->
        <TresGroup :position="[0, marqueeGroupY, marqueeGroupZ]">
          <TresMesh cast-shadow>
            <TresBoxGeometry :args="[mainBodyWidth * 0.9, marqueeH, marqueeD]" />
            <TresMeshToonMaterial color="#303030" />
          </TresMesh>
          <TresMesh :position-z="marqueeD / 2 + 0.01">
            <TresPlaneGeometry :args="[mainBodyWidth * 0.88, marqueeH * 0.8]" />
            <TresMeshStandardMaterial
              color="#FFFF99"
              :emissive="'#FFEE77'"
              :emissive-intensity="0.9"
            />
          </TresMesh>
        </TresGroup>
        <TresGroup :position="[0, controlsSurfaceY, controlsGroupZ]">
          <TresGroup :position="[-controlPanelBaseWidth * 0.25, 0.05, 0]">
            <TresMesh :position-y="0.025" cast-shadow>
              <TresCylinderGeometry :args="[0.12, 0.15, 0.05, 16]" />
              <TresMeshToonMaterial color="#303030" />
            </TresMesh>
            <TresMesh :position="[0, 0.225, 0]" cast-shadow>
              <TresCylinderGeometry :args="[0.04, 0.04, 0.35, 12]" />
              <TresMeshToonMaterial color="#222222" />
            </TresMesh>
            <TresMesh :position="[0, 0.225 + 0.175 + 0.06, 0]" cast-shadow>
              <TresSphereGeometry :args="[0.08, 16, 16]" />
              <TresMeshToonMaterial color="#FF3333" />
            </TresMesh>
          </TresGroup>
          <TresGroup :position="[controlPanelBaseWidth * 0.1, 0, 0.05]">
            <TresMesh :position="[0, 0.04, 0]" cast-shadow>
              <TresCylinderGeometry :args="[0.09, 0.09, 0.08, 16]" />
              <TresMeshToonMaterial color="#33FF33" />
            </TresMesh>
            <TresMesh :position="[0, 0.02, 0]" cast-shadow>
              <TresCylinderGeometry :args="[0.1, 0.1, 0.04, 16]" />
              <TresMeshToonMaterial color="#252525" />
            </TresMesh>
            <TresMesh :position="[0.22, 0.04, 0]" cast-shadow>
              <TresCylinderGeometry :args="[0.09, 0.09, 0.08, 16]" />
              <TresMeshToonMaterial color="#3333FF" />
            </TresMesh>
            <TresMesh :position="[0.22, 0.02, 0]" cast-shadow>
              <TresCylinderGeometry :args="[0.1, 0.1, 0.04, 16]" />
              <TresMeshToonMaterial color="#252525" />
            </TresMesh>
            <TresMesh :position="[0.44, 0.04, 0]" cast-shadow>
              <TresBoxGeometry :args="[0.15, 0.08, 0.08]" />
              <TresMeshToonMaterial color="#DDDD33" />
            </TresMesh>
            <TresMesh :position="[0.44, 0.02, 0]" cast-shadow>
              <TresBoxGeometry :args="[0.17, 0.04, 0.1]" />
              <TresMeshToonMaterial color="#252525" />
            </TresMesh>
          </TresGroup>
        </TresGroup>
      </TresGroup>

      <TresDirectionalLight
        :position="[cameraPosition.x * 0.8, cameraPosition.y * 1.5, cameraPosition.z * 0.7]"
        :intensity="1.8"
        cast-shadow
        :shadow-camera-far="50"
        :shadow-camera-left="-10"
        :shadow-camera-right="10"
        :shadow-camera-top="10"
        :shadow-camera-bottom="-10"
        :shadow-bias="-0.001"
        :shadow-mapSize-width="2048"
        :shadow-mapSize-height="2048"
      />
      <TresAmbientLight :intensity="0.9" />
    </TresCanvas>
  </div>
</template>

<style>
/* Keep your existing global styles */
html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

#app {
  height: 100%;
  width: 100%;
  background-color: #1a1a1a;
}

/* NEW: Styles for the container and hidden UI */
.scene-container {
  position: relative;
  width: 100%;
  height: 100%;
}

.ui-overlay {
  /* This positions the UI off-screen so it's not visible but can be rendered */
  position: absolute;
  top: 0;
  left: -9999px;
  z-index: -1;
}
</style>
