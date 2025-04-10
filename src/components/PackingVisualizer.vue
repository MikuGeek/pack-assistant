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

  // Add axes helper
  // const axesHelper = new THREE.AxesHelper(maxDim);
  // scene.add(axesHelper);

  // Draw the box and items
  drawBoxAndItems();

  // Animation loop
  animate();
}

function drawBoxAndItems() {
  if (!scene || !props.box) return;

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

  const boxGeom = new THREE.BoxGeometry(props.box.length, props.box.height, props.box.width); // Note: Y is height in Three.js
  const boxEdges = new THREE.EdgesGeometry(boxGeom);
  const boxLines = new THREE.LineSegments(boxEdges, new THREE.LineBasicMaterial({ color: 0x333333 })); // Dark lines for box outline
  // Position the box so its base is at y=0
  boxLines.position.set(0, props.box.height / 2, 0);
  scene.add(boxLines);

  // Add items
  const itemColor = new THREE.Color(getBoxColor(props.boxIndex));
  props.box.items.forEach(item => {
    if (item.position) {
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
  });
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
    drawBoxAndItems();
    // Recenter camera view on new box
    const maxDim = Math.max(props.box.length, props.box.width, props.box.height);
    camera.position.set(maxDim * 1.5, maxDim, maxDim * 1.5);
    controls.target.set(0, props.box.height / 2, 0);
    controls.maxDistance = maxDim * 5;
    controls.update();
  } else if (!props.box && renderer) {
    // Clean up if box is removed
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