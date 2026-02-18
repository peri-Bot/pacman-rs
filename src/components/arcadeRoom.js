/**
 * arcadeRoom.js — Retro arcade room environment builder
 * Returns a Three.js Group containing the full room: checkered floor,
 * walls, ceiling, neon strip lights, background cabinets, and neon signage.
 */
import {
  Group,
  Mesh,
  PlaneGeometry,
  BoxGeometry,
  CylinderGeometry,
  MeshStandardMaterial,
  MeshBasicMaterial,
  PointLight,
  Color,
} from 'three'

// --- Room dimensions ---
const ROOM_W = 120
const ROOM_D = 100
const ROOM_H = 70
const FLOOR_Y = -0.5

// --- Colors ---
const FLOOR_DARK = '#0a0a18'
const FLOOR_LIGHT = '#12122a'
const WALL_COLOR = '#0c0c1e'
const CEILING_COLOR = '#060612'
const NEON_PINK = '#ff2d8a'
const NEON_CYAN = '#00e5ff'
const NEON_PURPLE = '#9933ff'

// --- Checkered floor ---
function buildFloor(group) {
  const tileSize = 5
  const tilesX = Math.ceil(ROOM_W / tileSize)
  const tilesZ = Math.ceil(ROOM_D / tileSize)

  for (let x = 0; x < tilesX; x++) {
    for (let z = 0; z < tilesZ; z++) {
      const isDark = (x + z) % 2 === 0
      const geo = new PlaneGeometry(tileSize, tileSize)
      const mat = new MeshStandardMaterial({
        color: isDark ? FLOOR_DARK : FLOOR_LIGHT,
        roughness: isDark ? 0.85 : 0.75,
        metalness: 0.05,
      })
      const tile = new Mesh(geo, mat)
      tile.rotation.x = -Math.PI / 2
      tile.position.set(
        (x - tilesX / 2) * tileSize + tileSize / 2,
        FLOOR_Y,
        (z - tilesZ / 2) * tileSize + tileSize / 2
      )
      tile.receiveShadow = true
      group.add(tile)
    }
  }
}

// --- Walls ---
function buildWalls(group) {
  const wallMat = new MeshStandardMaterial({
    color: WALL_COLOR,
    roughness: 0.85,
    metalness: 0.05,
  })

  // Back wall
  const backWall = new Mesh(new PlaneGeometry(ROOM_W, ROOM_H), wallMat)
  backWall.position.set(0, ROOM_H / 2 + FLOOR_Y, -ROOM_D / 2)
  group.add(backWall)

  // Left wall
  const leftWall = new Mesh(new PlaneGeometry(ROOM_D, ROOM_H), wallMat.clone())
  leftWall.rotation.y = Math.PI / 2
  leftWall.position.set(-ROOM_W / 2, ROOM_H / 2 + FLOOR_Y, 0)
  group.add(leftWall)

  // Right wall
  const rightWall = new Mesh(new PlaneGeometry(ROOM_D, ROOM_H), wallMat.clone())
  rightWall.rotation.y = -Math.PI / 2
  rightWall.position.set(ROOM_W / 2, ROOM_H / 2 + FLOOR_Y, 0)
  group.add(rightWall)
}

// --- Ceiling ---
function buildCeiling(group) {
  const ceilMat = new MeshStandardMaterial({
    color: CEILING_COLOR,
    roughness: 0.95,
    metalness: 0,
  })
  const ceiling = new Mesh(new PlaneGeometry(ROOM_W, ROOM_D), ceilMat)
  ceiling.rotation.x = Math.PI / 2
  ceiling.position.set(0, ROOM_H + FLOOR_Y, 0)
  group.add(ceiling)
}

// --- Neon strip lights on side walls (horizontal) ---
function buildNeonStrips(group) {
  const stripLength = ROOM_D * 0.7
  const stripY = ROOM_H * 0.55 + FLOOR_Y

  // Collect pink flicker references
  const flickerItems = { tubes: [], halos: [], lights: [] }

  const strips = [
    // Left wall — pink strip (horizontal, runs along Z)
    { wallX: -ROOM_W / 2 + 1, y: stripY, color: NEON_PINK, flicker: true },
    // Right wall — cyan strip (horizontal, above ARCADE sign)
    { wallX: ROOM_W / 2 - 1, y: ROOM_H * 0.75 + FLOOR_Y, color: NEON_CYAN, flicker: false },
  ]

  strips.forEach(({ wallX, y, color, flicker }) => {
    // Glowing tube (horizontal, runs along Z axis)
    const tubeMat = new MeshBasicMaterial({ color })
    const tube = new Mesh(new CylinderGeometry(0.3, 0.3, stripLength, 8), tubeMat)
    tube.rotation.x = Math.PI / 2 // lay it along Z
    tube.position.set(wallX, y, 0)
    group.add(tube)

    // Glow halo
    const haloMat = new MeshBasicMaterial({
      color,
      transparent: true,
      opacity: 0.15,
    })
    const halo = new Mesh(new CylinderGeometry(1.8, 1.8, stripLength, 8), haloMat)
    halo.rotation.x = Math.PI / 2
    halo.position.copy(tube.position)
    group.add(halo)

    // Point lights along the strip for room illumination
    const stripLights = []
    for (let z = -stripLength * 0.4; z <= stripLength * 0.4; z += 18) {
      const light = new PointLight(new Color(color), 40, 130, 1)
      light.position.set(wallX + (wallX > 0 ? -6 : 6), y, z)
      group.add(light)
      stripLights.push(light)
    }

    // Floor-level pool lights — cast color onto the floor
    for (let z = -stripLength * 0.3; z <= stripLength * 0.3; z += 25) {
      const floorLight = new PointLight(new Color(color), 20, 70, 1)
      floorLight.position.set(wallX + (wallX > 0 ? -15 : 15), FLOOR_Y + 2, z)
      group.add(floorLight)
      stripLights.push(floorLight)
    }

    if (flicker) {
      flickerItems.tubes.push(tube)
      flickerItems.halos.push(halo)
      flickerItems.lights.push(...stripLights)
    }
  })

  return flickerItems
}

// --- Wall accent strips (glowing base strips) ---
function buildWallAccents(group) {
  const accentHeight = 0.4
  const accentMat = new MeshBasicMaterial({
    color: NEON_PURPLE,
    transparent: true,
    opacity: 0.6,
  })

  const backAccent = new Mesh(new BoxGeometry(ROOM_W, accentHeight, 0.3), accentMat)
  backAccent.position.set(0, FLOOR_Y + accentHeight / 2, -ROOM_D / 2 + 0.2)
  group.add(backAccent)

  const leftAccent = new Mesh(new BoxGeometry(0.3, accentHeight, ROOM_D), accentMat.clone())
  leftAccent.position.set(-ROOM_W / 2 + 0.2, FLOOR_Y + accentHeight / 2, 0)
  group.add(leftAccent)

  const rightAccent = new Mesh(new BoxGeometry(0.3, accentHeight, ROOM_D), accentMat.clone())
  rightAccent.position.set(ROOM_W / 2 - 0.2, FLOOR_Y + accentHeight / 2, 0)
  group.add(rightAccent)

  // Accent glow lights
  const positions = [
    [-ROOM_W / 2 + 1, FLOOR_Y + 2, 0],
    [ROOM_W / 2 - 1, FLOOR_Y + 2, 0],
    [0, FLOOR_Y + 2, -ROOM_D / 2 + 1],
  ]
  positions.forEach(([x, y, z]) => {
    const light = new PointLight(new Color(NEON_PURPLE), 3, 25, 2)
    light.position.set(x, y, z)
    group.add(light)
  })
}


// --- Neon "ARCADE" sign on right side wall ---
function buildNeonSign(group) {
  const signCenterY = ROOM_H * 0.55 + FLOOR_Y
  const wallX = ROOM_W / 2 - 1

  // Sign backing on right wall
  const backingMat = new MeshStandardMaterial({
    color: '#111122',
    roughness: 0.8,
    metalness: 0.2,
  })
  const backing = new Mesh(new BoxGeometry(0.5, 6, 30), backingMat)
  backing.position.set(wallX, signCenterY, 0)
  group.add(backing)

  // Neon letters as glowing bars — laid out along Z axis on right wall
  const letterMat = new MeshBasicMaterial({ color: NEON_CYAN })
  const letters = [
    // A
    { d: 0.6, h: 4, z: -11 },
    { d: 0.6, h: 4, z: -9 },
    { d: 2.6, h: 0.6, z: -10, y: 2 },
    { d: 2.6, h: 0.6, z: -10, y: 0 },
    // R  (vertical stem + top bar + middle bar + right stem top half + diagonal leg)
    { d: 0.6, h: 4, z: -6 },
    { d: 2.6, h: 0.6, z: -5, y: 2 },
    { d: 2.6, h: 0.6, z: -5, y: 0 },
    { d: 0.6, h: 2, z: -3.4, y: 1 },
    { d: 0.6, h: 2.2, z: -3.7, y: -1 },
    // C
    { d: 0.6, h: 4, z: -1 },
    { d: 2.6, h: 0.6, z: 0, y: 2 },
    { d: 2.6, h: 0.6, z: 0, y: -1.7 },
    // A
    { d: 0.6, h: 4, z: 3 },
    { d: 0.6, h: 4, z: 5 },
    { d: 2.6, h: 0.6, z: 4, y: 2 },
    { d: 2.6, h: 0.6, z: 4, y: 0 },
    // D
    { d: 0.6, h: 4, z: 7 },
    { d: 0.6, h: 3, z: 9, y: 0.2 },
    { d: 2.6, h: 0.6, z: 8, y: 2 },
    { d: 2.6, h: 0.6, z: 8, y: -1.7 },
    // E
    { d: 0.6, h: 4, z: 11 },
    { d: 2.6, h: 0.6, z: 12, y: 2 },
    { d: 2.0, h: 0.6, z: 11.7, y: 0 },
    { d: 2.6, h: 0.6, z: 12, y: -1.7 },
  ]

  letters.forEach(({ d, h, z, y = 0 }) => {
    const bar = new Mesh(new BoxGeometry(0.3, h, d), letterMat)
    bar.position.set(wallX - 0.4, signCenterY + y, z)
    group.add(bar)
  })

  // Sign glow
  const signGlow = new PointLight(new Color(NEON_CYAN), 12, 40, 2)
  signGlow.position.set(wallX - 5, signCenterY, 0)
  group.add(signGlow)
}

// --- Additional room lights ---
function buildRoomLights(group) {
  const spotWarm = new PointLight(new Color('#ffaa44'), 5, 50, 2)
  spotWarm.position.set(0, ROOM_H - 5 + FLOOR_Y, 15)
  group.add(spotWarm)
}

/**
 * Build the complete arcade room and return the Group.
 * Caller adds it to the scene and is responsible for cleanup.
 */
export function buildArcadeRoom() {
  const room = new Group()
  room.name = 'ArcadeRoom'

  buildFloor(room)
  buildWalls(room)
  buildCeiling(room)
  const flickerItems = buildNeonStrips(room)
  buildWallAccents(room)
  buildNeonSign(room)
  buildRoomLights(room)

  return { room, flickerItems }
}
