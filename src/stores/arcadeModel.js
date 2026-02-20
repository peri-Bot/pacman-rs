import { shallowRef } from 'vue'

// Shared store for the loaded glTF model — loaded once in LoadingScreen, reused in ArcadeScene
const modelScene = shallowRef(null)
const modelAnimations = shallowRef([])

export function useArcadeModel() {
    function setModel(gltf) {
        modelScene.value = gltf.scene
        modelAnimations.value = gltf.animations || []
    }

    return {
        modelScene,
        modelAnimations,
        setModel,
    }
}
