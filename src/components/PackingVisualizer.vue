<script setup lang="ts">
import { ref, onMounted, watch, nextTick } from 'vue';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

// Type definitions mirroring App.vue for clarity, though ideally shared
interface Item {
  id: string;
  destination: string;
  length: number;
  width: number;
  height: number;
  weight: number;
  position?: [number, number, number]; // Position within the box [x, y, z]
  box_index?: number;
}

interface PackedBox {
  items: Item[];
  length: number;
  width: number;
  height: number;
  weight: number;
  destination: string;
}

const props = defineProps<{
  box: PackedBox | null;
  boxIndex: number;
}>();

const containerRef = ref<HTMLDivElement | null>(null);
let scene: THREE.Scene;
let camera: THREE.PerspectiveCamera;
let renderer: THREE.WebGLRenderer;
let controls: OrbitControls;
const boxColors = ["#a7f3d0", "#bfdbfe", "#fef08a", "#fecaca", "#e9d5ff", "#fbcfe8"]; // Tailwind green-200, blue-200, yellow-200, red-200, purple-200, pink-200
const animationQueue = ref<Item[]>([]); // Queue of items to be animated
const animationSpeed = ref(300); // ms between item animations
let animationTimeout: number | null = null;

// Add reset and controls for animation
const isAnimating = ref(false);
const showAnimationControls = ref(true);
const animationProgress = ref(0);

function getBoxColor(index: number): string {
  return boxColors[index % boxColors.length];
}

function initScene() {
  if (!containerRef.value || !props.box) return;

  // Scene setup
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0xf0f0f0); // Light gray background

  // Camera setup
  const aspect = containerRef.value.clientWidth / containerRef.value.clientHeight;
  camera = new THREE.PerspectiveCamera(50, aspect, 0.1, 1000);
  // Position camera further back and slightly above to view the box
  const maxDim = Math.max(props.box.length, props.box.width, props.box.height);
  camera.position.set(maxDim * 1.5, maxDim, maxDim * 1.5);
  camera.lookAt(0, props.box.height / 2, 0); // Look towards the center of the box volume

  // Renderer setup
  renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight);
  containerRef.value.appendChild(renderer.domElement);

  // Lighting
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
  scene.add(ambientLight);
  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
  directionalLight.position.set(50, 100, 75);
  scene.add(directionalLight);

  // Controls
  controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true; // Smooth camera movement
  controls.dampingFactor = 0.05;
  controls.screenSpacePanning = false;
  controls.minDistance = 10;
  controls.maxDistance = maxDim * 5; // Adjust max distance based on box size
  controls.target.set(0, props.box.height / 2, 0); // Target the center of the box volume
  controls.update();

  // Add grid helper for context
  const gridHelper = new THREE.GridHelper(maxDim * 2, 20);
  scene.add(gridHelper);

  // Draw the empty box
  drawEmptyBox();

  // Setup animation queue
  setupAnimation();

  // Animation loop
  animate();
}

function drawEmptyBox() {
  if (!scene || !props.box) return;

  // Clear the scene first
  clearScene();

  // Draw just the box
  const boxGeom = new THREE.BoxGeometry(props.box.length, props.box.height, props.box.width); // Note: Y is height in Three.js
  const boxEdges = new THREE.EdgesGeometry(boxGeom);
  const boxLines = new THREE.LineSegments(boxEdges, new THREE.LineBasicMaterial({ color: 0x333333 })); // Dark lines for box outline
  // Position the box so its base is at y=0
  boxLines.position.set(0, props.box.height / 2, 0);
  scene.add(boxLines);
}

function clearScene() {
  if (!scene) return;

  // Better way to clear previous objects - remove all meshes and line segments
  // Keep only lights, cameras, and grid helper
  const objectsToRemove: THREE.Object3D[] = [];
  scene.traverse((child) => {
    if (child instanceof THREE.Mesh ||
        (child instanceof THREE.LineSegments && !(child instanceof THREE.GridHelper))) {
      objectsToRemove.push(child);
    }
  });

  // Remove all identified objects
  objectsToRemove.forEach(obj => {
    scene.remove(obj);
    // Also dispose geometries and materials to prevent memory leaks
    if (obj instanceof THREE.Mesh) {
      if (obj.geometry) obj.geometry.dispose();
      if (obj.material) {
        if (Array.isArray(obj.material)) {
          obj.material.forEach(mat => mat.dispose());
        } else {
          obj.material.dispose();
        }
      }
    } else if (obj instanceof THREE.LineSegments) {
      if (obj.geometry) obj.geometry.dispose();
      if (obj.material) {
        if (Array.isArray(obj.material)) {
          obj.material.forEach(mat => mat.dispose());
        } else {
          obj.material.dispose();
        }
      }
    }
  });
}

function setupAnimation() {
  if (!props.box) return;

  // Clear any existing animation
  stopAnimation();

  // Setup the animation queue with items that have positions
  animationQueue.value = [...props.box.items.filter(item => item.position)];
  animationProgress.value = 0;

  // Draw the empty box
  drawEmptyBox();
}

function startAnimation() {
  if (!props.box || animationQueue.value.length === 0) return;

  isAnimating.value = true;
  animateNextItem();
}

function stopAnimation() {
  isAnimating.value = false;
  if (animationTimeout !== null) {
    window.clearTimeout(animationTimeout);
    animationTimeout = null;
  }
}

function resetAnimation() {
  stopAnimation();
  setupAnimation();
}

function animateNextItem() {
  if (!scene || !props.box || !isAnimating.value || animationQueue.value.length === 0) return;

  const item = animationQueue.value.shift();
  if (!item || !item.position) return;

  addItemToScene(item);
  animationProgress.value = props.box.items.length - animationQueue.value.length;

  if (animationQueue.value.length > 0) {
    animationTimeout = window.setTimeout(() => {
      animateNextItem();
    }, animationSpeed.value);
  } else {
    isAnimating.value = false;
  }
}

function addItemToScene(item: Item) {
  if (!scene || !props.box || !item.position) return;

  const itemColor = new THREE.Color(getBoxColor(props.boxIndex));
  const itemGeom = new THREE.BoxGeometry(item.length, item.height, item.width); // length=x, height=y, width=z
  const itemMaterial = new THREE.MeshLambertMaterial({ color: itemColor, opacity: 0.85, transparent: true });
  const itemMesh = new THREE.Mesh(itemGeom, itemMaterial);

  // Calculate position relative to the box center for Three.js
  // The packing algorithm's [0,0,0] is a corner, Three.js's is the center.
  // Also, the packing alg uses (length, width, height), Three uses (x, y, z) where y is height.
  const x = item.position[0] + item.length / 2 - props.box!.length / 2;
  const y = item.position[2] + item.height / 2; // Pack height maps to Three.js y
  const z = item.position[1] + item.width / 2 - props.box!.width / 2; // Pack width maps to Three.js z
  itemMesh.position.set(x, y, z);

  scene.add(itemMesh);

  // Add edges to the item for better visibility
  const itemEdges = new THREE.EdgesGeometry(itemGeom);
  const itemLines = new THREE.LineSegments(itemEdges, new THREE.LineBasicMaterial({ color: 0x555555, linewidth: 0.5 })); // Fainter lines for items
  itemLines.position.copy(itemMesh.position); // Position edges with the mesh
  scene.add(itemLines);
}

function animate() {
  if (!renderer || !scene || !camera) return;
  requestAnimationFrame(animate);
  controls.update(); // Required if damping or auto-rotation is enabled
  renderer.render(scene, camera);
}

function handleResize() {
    if (!containerRef.value || !camera || !renderer) return;
    const width = containerRef.value.clientWidth;
    const height = containerRef.value.clientHeight;

    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    renderer.setSize(width, height);
}

onMounted(() => {
  nextTick(() => {
    if (containerRef.value && props.box) {
      initScene();
      window.addEventListener('resize', handleResize);
    }
  });
});

watch(() => [props.box, props.boxIndex], () => {
  if (props.box && scene) {
    // Stop any ongoing animation
    stopAnimation();
    // Setup the scene with empty box
    setupAnimation();

    // Recenter camera view on new box
    const maxDim = Math.max(props.box.length, props.box.width, props.box.height);
    camera.position.set(maxDim * 1.5, maxDim, maxDim * 1.5);
    controls.target.set(0, props.box.height / 2, 0);
    controls.maxDistance = maxDim * 5;
    controls.update();
  } else if (!props.box && renderer) {
    // Clean up if box is removed
    stopAnimation();
    if (containerRef.value) {
        containerRef.value.removeChild(renderer.domElement);
    }
    renderer.dispose();
    window.removeEventListener('resize', handleResize);
  }
}, { deep: true });

// Cleanup on unmount
import { onUnmounted } from 'vue';
onUnmounted(() => {
    stopAnimation();
    window.removeEventListener('resize', handleResize);
    if (renderer) {
        renderer.dispose(); // Dispose renderer resources
    }
    if (controls) {
        controls.dispose(); // Dispose controls
    }
    // Clear any remaining Three.js resources
    if (scene) {
      scene.traverse((object) => {
        if (object instanceof THREE.Mesh) {
          if (object.geometry) object.geometry.dispose();
          if (object.material) {
            if (Array.isArray(object.material)) {
              object.material.forEach((material) => material.dispose());
            } else {
              object.material.dispose();
            }
          }
        }
      });
    }
});


</script>

<template>
  <div ref="containerRef" class="w-full h-64 md:h-96 border rounded-lg bg-gray-50 overflow-hidden relative">
    <div v-if="!box" class="absolute inset-0 flex items-center justify-center text-gray-400">
      Select a box to view visualization
    </div>
    <div v-if="box && showAnimationControls" class="absolute bottom-2 left-2 right-2 p-2 bg-white bg-opacity-80 rounded-lg flex flex-col md:flex-row gap-2 items-center">
      <div class="flex space-x-2">
        <button
          @click="startAnimation"
          :disabled="isAnimating || animationQueue.length === 0"
          class="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed text-sm"
        >
          Play
        </button>
        <button
          @click="stopAnimation"
          :disabled="!isAnimating"
          class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 disabled:opacity-50 disabled:cursor-not-allowed text-sm"
        >
          Stop
        </button>
        <button
          @click="resetAnimation"
          class="px-3 py-1 bg-gray-500 text-white rounded hover:bg-gray-600 text-sm"
        >
          Reset
        </button>
      </div>

      <div class="flex items-center gap-2 flex-1">
        <label class="text-sm text-gray-600">Speed:</label>
        <input
          type="range"
          v-model="animationSpeed"
          min="50"
          max="1000"
          step="50"
          class="flex-1"
        />
      </div>

      <div class="text-sm text-gray-600">
        {{ animationProgress }} / {{ box.items.length }} items
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Ensure the canvas takes up the full container size */
:deep(canvas) {
  display: block;
  width: 100%;
  height: 100%;
}
</style>