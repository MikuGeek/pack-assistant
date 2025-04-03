<script setup lang="ts">
import { ref, computed, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";

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

// Available destinations
const destinations = ["Australia", "USA", "UK", "Germany", "Japan"];

// Form state for new item
const newItem = reactive<Item>({
  id: "",
  destination: "Australia",
  length: 0,
  width: 0,
  height: 0,
  weight: 0,
});

// Items list
const items = ref<Item[]>([]);

// Solution data
const solution = ref<PackingSolution | null>(null);
const isCalculating = ref(false);
const showResult = ref(false);

// For example items
const exampleItems: Record<string, { id: string; length: number; width: number; height: number; weight: number }> = {
  Australia: { id: "AUN11-ZY0009", length: 30, width: 20, height: 8, weight: 0.68 },
  USA: { id: "USY6-05-0314", length: 49, width: 21, height: 8, weight: 6.999 },
  UK: { id: "UKB8-ZY0009", length: 30, width: 20, height: 8, weight: 0.68 },
  Germany: { id: "DEW1-ZB0004", length: 45, width: 29, height: 17, weight: 1.5 },
  Japan: { id: "JPY5-GR049", length: 25, width: 25, height: 10, weight: 0.82 },
};

// Validation state
const errors = ref<Record<string, string>>({});

// Add item to the list
function addItem() {
  // Validate input
  const newErrors: Record<string, string> = {};

  if (!newItem.id) newErrors.id = "ID is required";
  if (newItem.length <= 0) newErrors.length = "Length must be greater than 0";
  if (newItem.width <= 0) newErrors.width = "Width must be greater than 0";
  if (newItem.height <= 0) newErrors.height = "Height must be greater than 0";
  if (newItem.weight <= 0) newErrors.weight = "Weight must be greater than 0";

  errors.value = newErrors;

  // If there are errors, don't add item
  if (Object.keys(newErrors).length > 0) return;

  // Add item to the list
  items.value.push({...newItem});

  // Reset form
  newItem.id = "";
  newItem.length = 0;
  newItem.width = 0;
  newItem.height = 0;
  newItem.weight = 0;
}

// Load example item
function loadExample() {
  const example = exampleItems[newItem.destination];
  newItem.id = example.id;
  newItem.length = example.length;
  newItem.width = example.width;
  newItem.height = example.height;
  newItem.weight = example.weight;
}

// Remove item from the list
function removeItem(index: number) {
  items.value.splice(index, 1);
}

// Calculate packing solution
async function calculatePacking() {
  if (items.value.length === 0) {
    alert("Please add at least one item to pack");
    return;
  }

  isCalculating.value = true;
  showResult.value = false;

  try {
    solution.value = await invoke("pack_items", { items: items.value });
    showResult.value = true;
  } catch (error) {
    console.error("Error calculating packing solution:", error);
    alert("Error calculating packing solution. Please check the console for details.");
  } finally {
    isCalculating.value = false;
  }
}

// Clear all data
function clearAll() {
  items.value = [];
  solution.value = null;
  showResult.value = false;
}

// Helper functions for display
function getBoxColor(index: number): string {
  const colors = ["bg-blue-200", "bg-green-200", "bg-yellow-200", "bg-red-200", "bg-purple-200", "bg-pink-200"];
  return colors[index % colors.length];
}

// Get constraints for destination
function getConstraints(destination: string) {
  const constraints: Record<string, any> = {
    "Australia": { dimensions: "Max 63cm per side", weight: "Max 22kg" },
    "USA": { dimensions: "Max 63cm per side", weight: "Max 22kg" },
    "UK": { dimensions: "Max 63cm per side", weight: "Max 15kg" },
    "Germany": { dimensions: "Max 63cm per side", weight: "Max 22.5kg" },
    "Japan": { dimensions: "Max 60cm length, 50cm width/height", weight: "Max 40kg" },
  };

  return constraints[destination] || { dimensions: "Unknown", weight: "Unknown" };
}

// Computed values
const totalItems = computed(() => items.value.length);
</script>

<template>
  <div class="min-h-screen bg-gray-50">
    <header class="bg-blue-600 text-white shadow-md">
      <div class="container mx-auto px-4 py-4">
        <h1 class="text-2xl font-bold">Packing Assistant</h1>
        <p class="text-sm">Optimize your shipping packages for cross-border e-commerce</p>
      </div>
    </header>

    <main class="container mx-auto px-4 py-6">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
        <!-- Left Column - Input Form -->
        <div class="lg:col-span-5 bg-white rounded-lg shadow-md p-6">
          <h2 class="text-xl font-semibold mb-4">Add Standard Items</h2>

          <!-- Item Form -->
          <div class="space-y-4">
            <div class="flex space-x-2">
              <div class="flex-1">
                <label class="block text-sm font-medium text-gray-700 mb-1">Item ID</label>
                <input
                  v-model="newItem.id"
                  type="text"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  :class="{'border-red-500': errors.id}"
                  placeholder="e.g., AUN11-ZY0009"
                />
                <p v-if="errors.id" class="text-red-500 text-xs mt-1">{{ errors.id }}</p>
              </div>

              <div class="w-1/2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Destination</label>
                <select
                  v-model="newItem.destination"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option v-for="dest in destinations" :key="dest" :value="dest">{{ dest }}</option>
                </select>
              </div>
            </div>

            <div class="grid grid-cols-4 gap-2">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Length (cm)</label>
                <input
                  v-model.number="newItem.length"
                  type="number"
                  min="0.1"
                  step="0.1"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  :class="{'border-red-500': errors.length}"
                />
                <p v-if="errors.length" class="text-red-500 text-xs mt-1">{{ errors.length }}</p>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Width (cm)</label>
                <input
                  v-model.number="newItem.width"
                  type="number"
                  min="0.1"
                  step="0.1"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  :class="{'border-red-500': errors.width}"
                />
                <p v-if="errors.width" class="text-red-500 text-xs mt-1">{{ errors.width }}</p>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Height (cm)</label>
                <input
                  v-model.number="newItem.height"
                  type="number"
                  min="0.1"
                  step="0.1"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  :class="{'border-red-500': errors.height}"
                />
                <p v-if="errors.height" class="text-red-500 text-xs mt-1">{{ errors.height }}</p>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Weight (kg)</label>
                <input
                  v-model.number="newItem.weight"
                  type="number"
                  min="0.001"
                  step="0.001"
                  class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  :class="{'border-red-500': errors.weight}"
                />
                <p v-if="errors.weight" class="text-red-500 text-xs mt-1">{{ errors.weight }}</p>
              </div>
            </div>

            <div class="flex space-x-2">
              <button
                @click="addItem"
                class="flex-1 bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                Add Item
              </button>
              <button
                @click="loadExample"
                class="bg-gray-200 text-gray-800 py-2 px-4 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-400"
              >
                Load Example
              </button>
            </div>
          </div>

          <div class="mt-6">
            <h3 class="text-lg font-medium mb-2">Items to Pack ({{ totalItems }})</h3>

            <div v-if="totalItems === 0" class="text-gray-500 text-center py-4 border rounded-md">
              No items added yet. Add items using the form above.
            </div>

            <div v-else class="border rounded-md overflow-hidden">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    <th class="px-4 py-2 text-xs font-medium text-gray-500 text-left">ID</th>
                    <th class="px-4 py-2 text-xs font-medium text-gray-500 text-left">Destination</th>
                    <th class="px-4 py-2 text-xs font-medium text-gray-500 text-left">Dimensions</th>
                    <th class="px-4 py-2 text-xs font-medium text-gray-500 text-left">Weight</th>
                    <th class="px-4 py-2 text-xs font-medium text-gray-500 text-left"></th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr v-for="(item, index) in items" :key="index">
                    <td class="px-4 py-2 text-sm">{{ item.id }}</td>
                    <td class="px-4 py-2 text-sm">{{ item.destination }}</td>
                    <td class="px-4 py-2 text-sm">{{ item.length }}×{{ item.width }}×{{ item.height }}cm</td>
                    <td class="px-4 py-2 text-sm">{{ item.weight }}kg</td>
                    <td class="px-4 py-2 text-sm">
                      <button
                        @click="removeItem(index)"
                        class="text-red-600 hover:text-red-800 focus:outline-none"
                      >
                        Remove
                      </button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>

            <div class="mt-4 flex space-x-2">
              <button
                @click="calculatePacking"
                class="flex-1 bg-green-600 text-white py-2 px-4 rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500"
                :disabled="isCalculating || totalItems === 0"
              >
                <span v-if="isCalculating">Calculating...</span>
                <span v-else>Calculate Optimal Packing</span>
              </button>
              <button
                @click="clearAll"
                class="bg-red-600 text-white py-2 px-4 rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
                :disabled="isCalculating || totalItems === 0"
              >
                Clear All
              </button>
            </div>
          </div>
        </div>

        <!-- Right Column - Results -->
        <div class="lg:col-span-7">
          <!-- Destination Info Card -->
          <div class="bg-white rounded-lg shadow-md p-6 mb-6">
            <h2 class="text-xl font-semibold mb-4">Destination Constraints</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              <div v-for="dest in destinations" :key="dest" class="border rounded-md p-3">
                <h3 class="text-base font-medium mb-1">{{ dest }}</h3>
                <p class="text-sm text-gray-600">{{ getConstraints(dest).dimensions }}</p>
                <p class="text-sm text-gray-600">{{ getConstraints(dest).weight }}</p>
              </div>
            </div>
          </div>

          <!-- Results Section -->
          <div v-if="showResult && solution" class="bg-white rounded-lg shadow-md p-6">
            <h2 class="text-xl font-semibold mb-4">Packing Solution</h2>

            <div class="mb-4 p-3 bg-blue-50 border border-blue-200 rounded-md">
              <p class="font-medium">Summary:</p>
              <ul class="list-disc ml-5 text-sm">
                <li>Total boxes: {{ solution.boxes.length }}</li>
                <li>Total volume: {{ solution.total_volume.toFixed(2) }} cm³</li>
                <li>Unpacked items: {{ solution.unpacked_items.length }}</li>
              </ul>
            </div>

            <!-- Packed Boxes -->
            <div v-if="solution.boxes.length > 0">
              <h3 class="text-lg font-medium mb-2">Packed Boxes</h3>

              <div class="space-y-4">
                <div
                  v-for="(box, boxIndex) in solution.boxes"
                  :key="boxIndex"
                  class="border rounded-md overflow-hidden"
                >
                  <div class="flex justify-between items-center bg-gray-50 px-4 py-2 border-b">
                    <div>
                      <span class="font-medium">Box #{{ boxIndex + 1 }} ({{ box.destination }})</span>
                      <span class="ml-2 text-sm text-gray-600">
                        {{ box.length.toFixed(1) }}×{{ box.width.toFixed(1) }}×{{ box.height.toFixed(1) }}cm
                      </span>
                    </div>
                    <div class="text-sm">
                      <span class="font-medium">Weight:</span> {{ box.weight.toFixed(2) }}kg
                    </div>
                  </div>

                  <div class="p-4">
                    <div class="max-h-64 overflow-auto">
                      <table class="min-w-full divide-y divide-gray-200">
                        <thead>
                          <tr>
                            <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">ID</th>
                            <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">Dimensions</th>
                            <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">Weight</th>
                            <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">Position</th>
                          </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-200">
                          <tr v-for="(item, itemIndex) in box.items" :key="itemIndex" :class="getBoxColor(boxIndex)">
                            <td class="px-3 py-2 text-sm">{{ item.id }}</td>
                            <td class="px-3 py-2 text-sm">{{ item.length }}×{{ item.width }}×{{ item.height }}cm</td>
                            <td class="px-3 py-2 text-sm">{{ item.weight }}kg</td>
                            <td class="px-3 py-2 text-sm">
                              <span v-if="item.position">
                                ({{ item.position[0].toFixed(1) }}, {{ item.position[1].toFixed(1) }}, {{ item.position[2].toFixed(1) }})
                              </span>
                            </td>
                          </tr>
                        </tbody>
                      </table>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Unpacked Items -->
            <div v-if="solution.unpacked_items.length > 0" class="mt-6">
              <h3 class="text-lg font-medium mb-2 text-red-600">Unpacked Items</h3>

              <div class="border border-red-200 rounded-md overflow-hidden">
                <table class="min-w-full divide-y divide-gray-200">
                  <thead class="bg-red-50">
                    <tr>
                      <th class="px-4 py-2 text-xs font-medium text-gray-700 text-left">ID</th>
                      <th class="px-4 py-2 text-xs font-medium text-gray-700 text-left">Destination</th>
                      <th class="px-4 py-2 text-xs font-medium text-gray-700 text-left">Dimensions</th>
                      <th class="px-4 py-2 text-xs font-medium text-gray-700 text-left">Weight</th>
                      <th class="px-4 py-2 text-xs font-medium text-gray-700 text-left">Reason</th>
                    </tr>
                  </thead>
                  <tbody class="bg-white divide-y divide-gray-200">
                    <tr v-for="(item, index) in solution.unpacked_items" :key="index">
                      <td class="px-4 py-2 text-sm">{{ item.id }}</td>
                      <td class="px-4 py-2 text-sm">{{ item.destination }}</td>
                      <td class="px-4 py-2 text-sm">{{ item.length }}×{{ item.width }}×{{ item.height }}cm</td>
                      <td class="px-4 py-2 text-sm">{{ item.weight }}kg</td>
                      <td class="px-4 py-2 text-sm text-red-600">
                        Exceeds destination constraints
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>

          <div v-else-if="!showResult && !isCalculating" class="bg-white rounded-lg shadow-md p-6 text-center">
            <div class="py-8">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
              <h3 class="mt-4 text-xl font-medium text-gray-700">No Packing Solution Yet</h3>
              <p class="mt-2 text-gray-500">Add items and click "Calculate Optimal Packing" to see the results.</p>
            </div>
          </div>

          <div v-else-if="isCalculating" class="bg-white rounded-lg shadow-md p-6 text-center">
            <div class="py-8">
              <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto"></div>
              <h3 class="mt-4 text-xl font-medium text-gray-700">Calculating Optimal Solution</h3>
              <p class="mt-2 text-gray-500">This may take a moment for complex packing requirements...</p>
            </div>
          </div>
        </div>
      </div>
    </main>

    <footer class="bg-gray-100 border-t mt-8">
      <div class="container mx-auto px-4 py-4 text-center text-gray-600 text-sm">
        Packing Assistant for Cross-Border E-commerce &copy; 2023
      </div>
    </footer>
  </div>
</template>

<style>
@import "tailwindcss";
</style>