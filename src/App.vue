<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import AppHeader from "./components/AppHeader.vue";
import AppFooter from "./components/AppFooter.vue";
import AddItemForm from "./components/AddItemForm.vue";
import ItemsList from "./components/ItemsList.vue";
import PackingResults from "./components/PackingResults.vue";
import PackingVisualizer from "./components/PackingVisualizer.vue";

// Types
interface Item {
  id: string;
  destination: string;
  length: number;
  width: number;
  height: number;
  weight: number;
  position?: [number, number, number];
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

interface PackingSolution {
  boxes: PackedBox[];
  total_volume: number;
  unpacked_items: Item[];
}

// Items list
const items = ref<Item[]>([]);

// Solution data
const solution = ref<PackingSolution | null>(null);
const isCalculating = ref(false);
const showResult = ref(false);
const selectedBoxIndex = ref<number | null>(null);

// Standard item definitions - predefined sizes and weights
const standardItems: Record<string, { id: string; name: string; length: number; width: number; height: number; weight: number; destination: string }> = {
  "AUN11-ZY0009": { id: "AUN11-ZY0009", name: "澳洲标准箱", length: 30, width: 20, height: 8, weight: 0.68, destination: "澳洲" },
  "USY6-05-0314": { id: "USY6-05-0314", name: "美国标准包裹", length: 49, width: 21, height: 8, weight: 6.999, destination: "美国" },
  "UKB8-ZY0009": { id: "UKB8-ZY0009", name: "英国标准包装", length: 30, width: 20, height: 8, weight: 0.68, destination: "英国" },
  "DEW1-ZB0004": { id: "DEW1-ZB0004", name: "德国标准箱", length: 45, width: 29, height: 17, weight: 1.5, destination: "德国" },
  "JPY5-GR049": { id: "JPY5-GR049", name: "日本标准包装", length: 25, width: 25, height: 10, weight: 0.82, destination: "日本" },
};

// Add items to the list
function addItems(itemId: string, quantity: number) {
  // Get the selected item
  const selectedItem = standardItems[itemId];

  // Add items to the list (one for each quantity)
  for (let i = 0; i < quantity; i++) {
    const newItemId = `${selectedItem.id}-${i+1}`;
    items.value.push({
      id: newItemId,
      destination: selectedItem.destination,
      length: selectedItem.length,
      width: selectedItem.width,
      height: selectedItem.height,
      weight: selectedItem.weight
    });
  }
}

// Remove item from the list
function removeItem(index: number) {
  items.value.splice(index, 1);
}

// Remove items by destination
function removeItemsByDestination(destination: string) {
  const indicesToRemove = items.value
    .map((item, index) => item.destination === destination ? index : -1)
    .filter(index => index !== -1)
    .sort((a, b) => b - a); // Sort in descending order to remove from end first

  for (const index of indicesToRemove) {
    items.value.splice(index, 1);
  }
}

// Calculate packing solution
async function calculatePacking() {
  if (items.value.length === 0) {
    alert("请至少添加一个物品进行打包");
    return;
  }

  isCalculating.value = true;
  showResult.value = false;

  try {
    solution.value = await invoke("pack_items", { items: items.value });
    showResult.value = true;
    if (solution.value) {
      selectedBoxIndex.value = solution.value.boxes.length > 0 ? 0 : null; // Select the first box by default
    }
  } catch (error) {
    console.error("Error calculating packing solution:", error);
    alert("计算打包方案时出错。请查看控制台了解详细信息。");
  } finally {
    isCalculating.value = false;
  }
}

// Clear all data
function clearAll() {
  items.value = [];
  solution.value = null;
  showResult.value = false;
  selectedBoxIndex.value = null;
}

// Select a box for visualization
function selectBox(index: number) {
  selectedBoxIndex.value = index;
}

// Computed property for the currently selected box object
const selectedBox = computed(() => {
  if (solution.value && selectedBoxIndex.value !== null && solution.value.boxes[selectedBoxIndex.value]) {
    return solution.value.boxes[selectedBoxIndex.value];
  }
  return null;
});
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <AppHeader @reset="clearAll" />

    <main class="container mx-auto px-4 py-6">
      <!-- Main Content Container with fixed width -->
      <div class="grid grid-cols-1 lg:grid-cols-7 gap-6 max-w-7xl mx-auto w-full">
        <!-- Left Panel - Item Creation -->
        <div class="lg:col-span-3 space-y-5">
          <!-- Add Items Form -->
          <AddItemForm :standard-items="standardItems" @add-items="addItems" />

          <!-- Items List -->
          <ItemsList
            :items="items"
            :standard-items="standardItems"
            @remove-item="removeItem"
            @remove-by-destination="removeItemsByDestination"
            @calculate-packing="calculatePacking"
            @clear-all="clearAll"
          />
        </div>

        <!-- Right Panel - Results -->
        <div class="lg:col-span-4 space-y-5">
          <!-- Visualization Card -->
          <div v-if="showResult && solution && selectedBoxIndex !== null" class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 w-full">
            <h2 class="text-lg font-semibold text-gray-800 mb-3 flex items-center">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
              </svg>
              箱子可视化 {{ selectedBoxIndex !== null ? `#${selectedBoxIndex + 1}` : '' }}
            </h2>
            <PackingVisualizer :box="selectedBox" :boxIndex="selectedBoxIndex ?? 0" />
          </div>

          <!-- Packing Results -->
          <PackingResults
            :solution="solution"
            :is-calculating="isCalculating"
            :show-result="showResult"
            :standard-items="standardItems"
            @select-box="selectBox"
          />
        </div>
      </div>
    </main>

    <AppFooter />
  </div>
</template>

<style>
@import "tailwindcss";
</style>