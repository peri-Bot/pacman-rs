<script setup>
import { useLoop, useTresContext } from '@tresjs/core'
import { OrbitControls } from '@tresjs/cientos'
import { Vector3, AnimationMixer, Box3 } from 'three'
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { gsap } from 'gsap'
import { useArcadeModel } from '../stores/arcadeModel'
import { buildArcadeRoom } from './arcadeRoom'

const emit = defineEmits(['zoomComplete', 'modelReady'])

// --- Refs ---
const controlsRef = ref(null)
const cameraRef = ref(null)

// --- Pre-loaded model from store ---
const { modelScene, modelAnimations } = useArcadeModel()

// --- Context ---
const tresContext = useTresContext()

// --- Animation Mixer ---
let mixer = null
let roomGroup = null
let flickerRefs = null
let flickerTime = 0

// Camera initial position — straight-on front view showing full cabinet with marquee
const initialCameraPos = [-11.0, 72.7, 77.6]
const initialLookAt = [0, 30, 0]

// --- Find the screen center in world space ---
function findScreenTarget() {
  if (!modelScene.value) return new Vector3(0, 35, 0)

  // Get the bounding box of the entire model to understand its scale
  const modelBox = new Box3().setFromObject(modelScene.value)
  const modelCenter = new Vector3()
  modelBox.getCenter(modelCenter)
  const modelSize = new Vector3()
  modelBox.getSize(modelSize)

  // The screen is roughly at the top-center-front of the arcade cabinet
  // Based on glTF data: the model is tall (Z axis in glTF → Y after rotation)
  // Screen is approximately at 60-70% height of the cabinet
  let screenTarget = null
  modelScene.value.traverse((child) => {
    if (child.name === 'scheibe' || child.name === 'scheibe_scheibe_0') {
      child.updateWorldMatrix(true, false)
      const pos = new Vector3()
      child.getWorldPosition(pos)
      screenTarget = pos
    }
  })

  if (screenTarget) {
    // The scheibe mesh center might be at origin due to local transforms
    // Use the model's bounding box to estimate screen position
    // Screen is typically at ~65% of the height, centered horizontally
    const screenY = modelBox.min.y + modelSize.y * 0.55
    return new Vector3(modelCenter.x, screenY, modelBox.min.z - 2)
  }

  // Fallback: aim at upper-center of model
  return new Vector3(modelCenter.x, modelCenter.y + modelSize.y * 0.1, modelBox.min.z)
}

// --- Setup animations ---
function setupAnimations() {
  if (!modelScene.value || modelAnimations.value.length === 0) return

  mixer = new AnimationMixer(modelScene.value)
  modelAnimations.value.forEach((clip) => {
    const action = mixer.clipAction(clip)
    action.play()
  })
}

// --- Get camera from multiple sources ---
function getCamera() {
  if (cameraRef.value) {
    const instance = cameraRef.value?.instance || cameraRef.value?.value || cameraRef.value
    if (instance && instance.isCamera) return instance
  }
  if (tresContext.camera?.value) return tresContext.camera.value
  if (tresContext.scene?.value) {
    let cam = null
    tresContext.scene.value.traverse((child) => {
      if (child.isCamera && !cam) cam = child
    })
    if (cam) return cam
  }
  return null
}

// --- Get OrbitControls instance ---
function getControls() {
  if (!controlsRef.value) return null
  const ref = controlsRef.value
  if (ref.value && ref.value.enabled !== undefined) return ref.value
  if (ref.instance && ref.instance.enabled !== undefined) return ref.instance
  if (ref.enabled !== undefined) return ref
  return null
}

// --- Log camera position on orbit change (for tinkering) ---
function onControlsChange() {
  const cam = getCamera()
  const controls = getControls()
  if (cam) {
    const pos = cam.position
    const target = controls?.target
    console.log(
      `📷 Camera pos: [${pos.x.toFixed(1)}, ${pos.y.toFixed(1)}, ${pos.z.toFixed(1)}]` +
        (target
          ? ` | target: [${target.x.toFixed(1)}, ${target.y.toFixed(1)}, ${target.z.toFixed(1)}]`
          : ''),
    )
  }
}

// --- Zoom animation ---
let zoomStarted = false

function startZoomAnimation() {
  if (zoomStarted) return
  zoomStarted = true
  removeInteractionListeners()

  const cam = getCamera()
  const controls = getControls()

  if (!cam) {
    zoomStarted = false
    setTimeout(() => startZoomAnimation(), 500)
    return
  }

  if (controls) {
    controls.enabled = false
  }

  const screenPos = findScreenTarget()
  const endPosition = new Vector3(screenPos.x, screenPos.y, screenPos.z + 22)

  const tl = gsap.timeline({
    onComplete: () => {
      emit('zoomComplete')
    },
  })

  tl.to(cam.position, {
    duration: 2.8,
    x: endPosition.x,
    y: endPosition.y,
    z: endPosition.z,
    ease: 'power3.inOut',
  })

  if (controls && controls.target) {
    tl.to(
      controls.target,
      {
        duration: 2.8,
        x: screenPos.x,
        y: screenPos.y,
        z: screenPos.z,
        ease: 'power3.inOut',
      },
      '<',
    )
  }
}

// --- User interaction listeners ---
function onKeyDown(e) {
  if (e.code === 'Space') {
    e.preventDefault()
    startZoomAnimation()
  }
}

function addInteractionListeners() {
  window.addEventListener('keydown', onKeyDown)
}

function removeInteractionListeners() {
  window.removeEventListener('keydown', onKeyDown)
}

// --- Render loop for animation mixer ---
const { onBeforeRender } = useLoop()
onBeforeRender(({ delta }) => {
  if (mixer) {
    mixer.update(delta)
  }

  // Pink neon flicker — broken light effect
  if (flickerRefs) {
    flickerTime += delta
    // Create an irregular flicker pattern
    const fast = Math.sin(flickerTime * 30) * 0.5 + 0.5
    const slow = Math.sin(flickerTime * 3.7) * 0.5 + 0.5
    const glitch = Math.random() > 0.92 ? 0 : 1 // occasional full blackout
    const flicker = Math.min(fast * 0.4 + slow * 0.6, 1) * glitch

    flickerRefs.tubes.forEach(t => { t.material.opacity = 0.3 + flicker * 0.7; t.material.transparent = true })
    flickerRefs.halos.forEach(h => { h.material.opacity = flicker * 0.15 })
    flickerRefs.lights.forEach(l => { l.intensity = flicker * 35 })
  }
})

// --- Use pre-loaded model on mount ---
onMounted(() => {
  if (!modelScene.value) {
    console.error('Arcade model not loaded — was the loading screen skipped?')
    return
  }

  // Build the arcade room environment
  const result = buildArcadeRoom()
  roomGroup = result.room
  flickerRefs = result.flickerItems
  tresContext.scene.value.add(roomGroup)

  // Add the pre-loaded arcade model
  tresContext.scene.value.add(modelScene.value)
  setupAnimations()

  emit('modelReady')
  addInteractionListeners()
})

onBeforeUnmount(() => {
  removeInteractionListeners()
  if (mixer) {
    mixer.stopAllAction()
    mixer = null
  }
  if (modelScene.value && tresContext.scene?.value) {
    tresContext.scene.value.remove(modelScene.value)
  }
  if (roomGroup && tresContext.scene?.value) {
    tresContext.scene.value.remove(roomGroup)
    roomGroup = null
  }
})
</script>

<template>
  <TresPerspectiveCamera
    ref="cameraRef"
    :position="initialCameraPos"
    :look-at="initialLookAt"
    :fov="50"
    :near="0.1"
    :far="500"
  />
  <OrbitControls
    ref="controlsRef"
    :target="initialLookAt"
    :enable-damping="true"
    :damping-factor="0.05"
    @change="onControlsChange"
  />

  <!-- Fog — softer for enclosed room -->
  <TresFog :args="['#080810', 80, 200]" />

  <!-- Lighting for the main cabinet -->
  <!-- Key spot from above-front -->
  <TresSpotLight
    :position="[0, 65, 35]"
    :intensity="120"
    :angle="0.5"
    :penumbra="0.6"
    :decay="1.5"
    color="#ffffff"
    cast-shadow
    :shadow-mapSize-width="2048"
    :shadow-mapSize-height="2048"
    :shadow-bias="-0.0001"
  />
  <!-- Fill light — illuminates the lower cabinet (coin slot, base art) -->
  <TresPointLight
    :position="[0, 10, 30]"
    :intensity="25"
    color="#ffe8cc"
    :decay="2"
  />
  <!-- Side fill — soft blue from left -->
  <TresPointLight
    :position="[-20, 30, 20]"
    :intensity="10"
    color="#6688ff"
    :decay="2"
  />
  <!-- Ambient base -->
  <TresAmbientLight :intensity="0.35" color="#1a1a2e" />
</template>
