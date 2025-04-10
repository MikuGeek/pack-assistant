<script setup lang="ts">
import { ref, computed, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PackingVisualizer from "./components/PackingVisualizer.vue"; // Import the visualizer

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

// Form state for adding items
const newItemForm = reactive({
  combinedSelection: "", // Combined destination and item ID
  quantity: 1,
});

// Items list
const items = ref<Item[]>([]);

// Solution data
const solution = ref<PackingSolution | null>(null);
const isCalculating = ref(false);
const showResult = ref(false);
const showDetailedList = ref(false);
const selectedBoxIndex = ref<number | null>(null); // State for selected box visualization

// Standard item definitions - predefined sizes and weights
const standardItems: Record<string, { id: string; name: string; length: number; width: number; height: number; weight: number; destination: string }> = {
  "AUN11-ZY0009": { id: "AUN11-ZY0009", name: "澳洲标准箱", length: 30, width: 20, height: 8, weight: 0.68, destination: "澳洲" },
  "USY6-05-0314": { id: "USY6-05-0314", name: "美国标准包裹", length: 49, width: 21, height: 8, weight: 6.999, destination: "美国" },
  "UKB8-ZY0009": { id: "UKB8-ZY0009", name: "英国标准包装", length: 30, width: 20, height: 8, weight: 0.68, destination: "英国" },
  "DEW1-ZB0004": { id: "DEW1-ZB0004", name: "德国标准箱", length: 45, width: 29, height: 17, weight: 1.5, destination: "德国" },
  "JPY5-GR049": { id: "JPY5-GR049", name: "日本标准包装", length: 25, width: 25, height: 10, weight: 0.82, destination: "日本" },
};

// Validation state
const errors = ref<Record<string, string>>({});

// Get all available item options
const availableItemOptions = computed(() =>
  Object.values(standardItems).map(item => ({
    value: item.id,
    label: `${item.destination} - ${item.name} (${item.length}×${item.width}×${item.height}cm, ${item.weight}kg)`,
  }))
);

// Add items to the list
function addItems() {
  // Validate input
  const newErrors: Record<string, string> = {};

  if (!newItemForm.combinedSelection) newErrors.combinedSelection = "请选择一个物品";
  if (newItemForm.quantity <= 0) newErrors.quantity = "数量必须大于0";

  errors.value = newErrors;

  // If there are errors, don't add items
  if (Object.keys(newErrors).length > 0) return;

  // Get the selected item
  const selectedItem = standardItems[newItemForm.combinedSelection];

  // Add items to the list (one for each quantity)
  for (let i = 0; i < newItemForm.quantity; i++) {
    const itemId = `${selectedItem.id}-${i+1}`;
    items.value.push({
      id: itemId,
      destination: selectedItem.destination,
      length: selectedItem.length,
      width: selectedItem.width,
      height: selectedItem.height,
      weight: selectedItem.weight
    });
  }

  // Reset quantity to 1 after adding
  newItemForm.quantity = 1;
}

// Remove item from the list
function removeItem(index: number) {
  items.value.splice(index, 1);
}

// Remove items by predicate
function removeItemsByPredicate(predicate: (item: Item) => boolean) {
  const indicesToRemove = items.value
    .map((item, index) => predicate(item) ? index : -1)
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
  selectedBoxIndex.value = null; // Clear selected box on clearAll
}

// Helper functions for display
function getBoxColor(index: number): string {
  const colors = ["bg-blue-200", "bg-green-200", "bg-yellow-200", "bg-red-200", "bg-purple-200", "bg-pink-200"];
  return colors[index % colors.length];
}

// Get constraints for destination
function getConstraints(destination: string) {
  const constraints: Record<string, any> = {
    "澳洲": {
      dimensions: "每边最大63cm",
      weight: "最大22kg",
      description: "外箱任何一边不得超过63cm，总重量必须低于22kg。"
    },
    "美国": {
      dimensions: "每边最大63cm",
      weight: "最大22kg",
      description: "外箱任何一边不得超过63cm，总重量必须低于22kg。"
    },
    "英国": {
      dimensions: "每边最大63cm",
      weight: "最大15kg",
      description: "外箱任何一边不得超过63cm，总重量必须低于15kg。"
    },
    "德国": {
      dimensions: "每边最大63cm",
      weight: "最大22.5kg",
      description: "外箱任何一边不得超过63cm，总重量必须低于22.5kg。"
    },
    "日本": {
      dimensions: "长度最大60cm，宽高最大50cm",
      weight: "最大40kg",
      description: "长度不得超过60cm，宽度和高度不得超过50cm，总重量必须低于40kg。"
    },
  };

  return constraints[destination] || { dimensions: "未知", weight: "未知", description: "无约束信息。" };
}

// Helper function to explain why an item couldn't be packed
function getUnpackedReason(item: Item): string {
  const constraints = getConstraints(item.destination);
  const maxDim = constraints.dimensions.includes("最大") ?
    parseInt(constraints.dimensions.match(/\d+/)?.[0] || "0") : 0;
  const maxWeight = constraints.weight.includes("最大") ?
    parseFloat(constraints.weight.match(/[\d\.]+/)?.[0] || "0") : 0;

  if (item.destination === "日本") {
    if (item.length > 60) return "长度超过60cm限制";
    if (item.width > 50) return "宽度超过50cm限制";
    if (item.height > 50) return "高度超过50cm限制";
    if (item.weight > 40) return "重量超过40kg限制";
  } else {
    if (item.length > maxDim) return `长度超过${maxDim}cm限制`;
    if (item.width > maxDim) return `宽度超过${maxDim}cm限制`;
    if (item.height > maxDim) return `高度超过${maxDim}cm限制`;
    if (item.weight > maxWeight) return `重量超过${maxWeight}kg限制`;
  }

  return "无法有效装入任何箱子";
}

// Computed values
const totalItems = computed(() => items.value.length);

// Organize items by destination
const itemsByDestination = computed(() => {
  const grouped: Record<string, Item[]> = {};

  items.value.forEach(item => {
    if (!grouped[item.destination]) {
      grouped[item.destination] = [];
    }
    grouped[item.destination].push(item);
  });

  return grouped;
});

// Get item name from standard item
function getItemName(itemId: string): string {
  const baseId = itemId.split('-').slice(0, -1).join('-');
  return standardItems[baseId]?.name || baseId;
}

// Calculate total weight of all items
const totalWeight = computed(() =>
  items.value.reduce((sum, item) => sum + item.weight, 0)
);

// Calculate total volume of all items
const totalVolume = computed(() =>
  items.value.reduce((sum, item) => sum + (item.length * item.width * item.height), 0)
);

// Calculate box space utilization
function calculateUtilization(box: PackedBox): number {
  const boxVolume = box.length * box.width * box.height;
  const itemsVolume = box.items.reduce((sum, item) => sum + (item.length * item.width * item.height), 0);
  return (itemsVolume / boxVolume) * 100;
}

// Function to select a box for visualization
function selectBoxForVisualization(index: number) {
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
    <header class="bg-indigo-700 text-white shadow-lg">
      <div class="container mx-auto px-4 py-5">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold">装箱助手</h1>
            <p class="text-indigo-100 text-sm">优化跨境电商物流装箱</p>
          </div>
          <div class="hidden md:block">
            <span class="px-3 py-1 bg-indigo-800 rounded-full text-xs">由 Tauri + Vue 驱动</span>
          </div>
        </div>
      </div>
    </header>

    <main class="container mx-auto px-4 py-6">
      <!-- Main Content Container with fixed width -->
      <div class="grid grid-cols-1 lg:grid-cols-7 gap-6 max-w-7xl mx-auto w-full">
        <!-- Left Panel - Item Creation -->
        <div class="lg:col-span-3 space-y-5">
          <!-- Add Items Card -->
          <div class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 w-full">
            <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-indigo-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
              </svg>
              添加物品
            </h2>

            <div class="space-y-4">
              <!-- Destination Selection -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">目的地和物品</label>
                <select
                  v-model="newItemForm.combinedSelection"
                  class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-colors"
                  :class="{'border-red-500 bg-red-50': errors.combinedSelection}"
                >
                  <option value="">选择目的地和物品</option>
                  <option v-for="option in availableItemOptions" :key="option.value" :value="option.value">{{ option.label }}</option>
                </select>
                <p v-if="errors.combinedSelection" class="text-red-500 text-xs mt-1">{{ errors.combinedSelection }}</p>
              </div>

              <!-- Quantity Input -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">数量</label>
                <input
                  v-model.number="newItemForm.quantity"
                  type="number"
                  min="1"
                  step="1"
                  class="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-colors"
                  :class="{'border-red-500 bg-red-50': errors.quantity}"
                />
                <p v-if="errors.quantity" class="text-red-500 text-xs mt-1">{{ errors.quantity }}</p>
              </div>

              <!-- Add Button -->
              <button
                @click="addItems"
                class="w-full bg-indigo-600 text-white py-2.5 px-4 rounded-lg hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 transition-colors font-medium"
              >
                添加到装箱清单
              </button>
            </div>
          </div>

          <!-- Items Container - Fixed height to prevent layout shifts -->
          <div class="min-h-[300px] w-full">
            <!-- Items Summary Card -->
            <div v-if="totalItems > 0" class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 w-full">
              <div class="flex justify-between items-center mb-4">
                <h2 class="text-lg font-semibold text-gray-800 flex items-center">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-indigo-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                  </svg>
                  物品概览
                </h2>
                <div class="flex space-x-1">
                  <button
                    @click="showDetailedList = !showDetailedList"
                    class="text-indigo-600 text-xs px-2 py-1 rounded hover:bg-indigo-50 transition-colors flex items-center"
                  >
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
                    </svg>
                    {{ showDetailedList ? '隐藏' : '显示' }}详情
                  </button>
                </div>
              </div>

              <!-- Summary Stats -->
              <div class="grid grid-cols-3 gap-2 mb-4">
                <div class="bg-indigo-50 rounded-lg p-3 text-center">
                  <div class="text-xs text-indigo-600 font-medium">物品数</div>
                  <div class="text-xl font-bold text-indigo-900">{{ totalItems }}</div>
                </div>
                <div class="bg-indigo-50 rounded-lg p-3 text-center">
                  <div class="text-xs text-indigo-600 font-medium">重量</div>
                  <div class="text-xl font-bold text-indigo-900">{{ totalWeight.toFixed(1) }}<span class="text-xs">kg</span></div>
                </div>
                <div class="bg-indigo-50 rounded-lg p-3 text-center">
                  <div class="text-xs text-indigo-600 font-medium">体积</div>
                  <div class="text-xl font-bold text-indigo-900">{{ (totalVolume / 1000).toFixed(1) }}<span class="text-xs">dm³</span></div>
                </div>
              </div>

              <!-- Tabs for different views -->
              <div class="border-b border-gray-200 mb-4">
                <div class="flex -mb-px">
                  <button
                    @click="showDetailedList = false"
                    class="px-4 py-2 text-sm font-medium border-b-2 focus:outline-none"
                    :class="!showDetailedList ? 'border-indigo-600 text-indigo-600' : 'border-transparent text-gray-500 hover:text-gray-700'"
                  >
                    概览
                  </button>
                  <button
                    @click="showDetailedList = true"
                    class="px-4 py-2 text-sm font-medium border-b-2 focus:outline-none"
                    :class="showDetailedList ? 'border-indigo-600 text-indigo-600' : 'border-transparent text-gray-500 hover:text-gray-700'"
                  >
                    详情
                  </button>
                </div>
              </div>

              <!-- Summary View -->
              <div v-if="!showDetailedList">
                <!-- Items by Destination -->
                <div class="mb-4">
                  <h3 class="text-sm font-medium text-gray-700 mb-2">发货目的地</h3>
                  <div class="space-y-2">
                    <div
                      v-for="(destItems, destination) in itemsByDestination"
                      :key="destination"
                      class="bg-white border border-gray-200 rounded-lg overflow-hidden shadow-sm"
                    >
                      <div class="bg-gradient-to-r from-indigo-50 to-white px-3 py-2 border-b font-medium flex justify-between items-center">
                        <span>{{ destination }}</span>
                        <span class="text-xs px-2 py-0.5 bg-indigo-100 rounded-full">{{ destItems.length }} 件</span>
                      </div>
                      <div class="px-3 py-2">
                        <div class="flex justify-between items-center">
                          <span class="text-sm text-gray-600">总重量：</span>
                          <span class="font-medium">
                            {{ destItems.reduce((sum, item) => sum + item.weight, 0).toFixed(2) }} kg
                          </span>
                        </div>
                        <div class="mt-2 flex justify-end">
                          <button
                            @click="removeItemsByPredicate(item => item.destination === destination)"
                            class="text-red-600 text-xs hover:text-red-800 focus:outline-none flex items-center"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-3 w-3 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                            全部删除
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Detailed View -->
              <div v-else>
                <div class="border rounded-lg overflow-hidden">
                  <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                      <tr>
                        <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">物品</th>
                        <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">目的地</th>
                        <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">尺寸/重量</th>
                        <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left"></th>
                      </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                      <tr v-for="(item, index) in items" :key="index" class="hover:bg-gray-50">
                        <td class="px-3 py-2 text-sm font-medium text-gray-600">
                          {{ getItemName(item.id) }}
                        </td>
                        <td class="px-3 py-2 text-sm text-gray-500">{{ item.destination }}</td>
                        <td class="px-3 py-2 text-sm text-gray-500">
                          {{ item.length }}×{{ item.width }}×{{ item.height }}cm, {{ item.weight }}kg
                        </td>
                        <td class="px-3 py-2 text-sm text-right">
                          <button
                            @click="removeItem(index)"
                            class="text-red-600 hover:text-red-800 focus:outline-none"
                          >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                          </button>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>

              <!-- Action Buttons -->
              <div class="mt-4 flex space-x-2">
                <button
                  @click="calculatePacking"
                  class="flex-1 bg-emerald-600 text-white py-2 px-4 rounded-lg hover:bg-emerald-700 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 transition-colors font-medium flex items-center justify-center"
                  :disabled="isCalculating || totalItems === 0"
                >
                  <svg v-if="!isCalculating" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  <svg v-else xmlns="http://www.w3.org/2000/svg" class="animate-spin h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                  </svg>
                  <span v-if="isCalculating">计算中...</span>
                  <span v-else>计算装箱方案</span>
                </button>
                <button
                  @click="clearAll"
                  class="bg-gray-200 text-gray-700 py-2 px-4 rounded-lg hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 transition-colors"
                  :disabled="isCalculating || totalItems === 0"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
            </div>

            <!-- Empty State - match the dimensions of the filled state -->
            <div v-if="totalItems === 0" class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 text-center w-full h-full flex flex-col justify-center">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12 mx-auto text-gray-300 mb-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
              </svg>
              <h3 class="text-gray-500 mb-1">未添加物品</h3>
              <p class="text-sm text-gray-400">添加物品以开始装箱优化</p>
            </div>
          </div>
        </div>

        <!-- Right Panel - Results -->
        <div class="lg:col-span-4 space-y-5">
          <!-- Results Container -->
          <div class="w-full">
            <!-- Results Section -->
            <div v-if="showResult && solution" class="space-y-5">
              <!-- Visualization Card -->
              <div class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 w-full">
                <h2 class="text-lg font-semibold text-gray-800 mb-3 flex items-center">
                   <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                     <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
                   </svg>
                  箱子可视化 {{ selectedBoxIndex !== null ? `#${selectedBoxIndex + 1}` : '' }}
                </h2>
                 <PackingVisualizer :box="selectedBox" :boxIndex="selectedBoxIndex ?? 0" />
              </div>

               <!-- Packing Solution Details Card -->
              <div class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 w-full">
                <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-emerald-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                  优化装箱方案
                </h2>

                <!-- Summary Card -->
                <div class="mb-4 p-3 bg-blue-50 border border-blue-200 rounded-md">
                  <p class="font-medium">概要：</p>
                  <ul class="list-disc ml-5 text-sm">
                    <li>总箱数：{{ solution.boxes.length }}</li>
                    <li>总体积：{{ solution.total_volume.toFixed(2) }} cm³</li>
                  </ul>
                </div>

                <!-- Packed Boxes -->
                <div v-if="solution.boxes.length > 0">
                  <h3 class="text-sm font-semibold uppercase tracking-wider text-gray-500 mb-3">已装箱物品（点击查看可视化）</h3>

                  <div class="space-y-4">
                    <div
                      v-for="(box, boxIndex) in solution.boxes"
                      :key="boxIndex"
                      class="border rounded-lg overflow-hidden shadow-sm transition-all duration-150 ease-in-out cursor-pointer"
                      :class="{ 'border-indigo-400 ring-2 ring-indigo-200': selectedBoxIndex === boxIndex, 'border-gray-200 hover:border-gray-300': selectedBoxIndex !== boxIndex }"
                      @click="selectBoxForVisualization(boxIndex)"
                    >
                      <!-- Box Header -->
                      <div class="flex justify-between items-center bg-gradient-to-r from-gray-50 to-white px-4 py-3 border-b" :class="{ 'bg-gradient-to-r from-indigo-50 to-indigo-100': selectedBoxIndex === boxIndex }">
                        <div>
                          <span class="font-medium text-gray-800">箱子 #{{ boxIndex + 1 }} • {{ box.destination }}</span>
                          <span class="ml-2 text-sm text-gray-500">
                            {{ (box.length + 1.2).toFixed(1) }}×{{ (box.width + 1.2).toFixed(1) }}×{{ (box.height + 1.2).toFixed(1) }}cm
                          </span>
                        </div>
                        <div class="text-sm">
                          <span class="font-medium text-gray-600">{{ box.weight.toFixed(2) }}kg</span>
                          <span class="ml-2 px-2 py-0.5 rounded-full text-xs" :class="selectedBoxIndex === boxIndex ? 'bg-indigo-200 text-indigo-800' : 'bg-gray-100 text-gray-700'">
                            装箱率 {{ calculateUtilization(box).toFixed(0) }}%
                          </span>
                        </div>
                      </div>

                      <!-- Box Content (Optional: could be hidden by default, shown on click/expand) -->
                      <div v-if="selectedBoxIndex === boxIndex" class="p-4 bg-white">
                         <div class="max-h-48 overflow-auto rounded-lg border border-gray-100">
                           <table class="min-w-full divide-y divide-gray-200">
                             <thead class="bg-gray-50 sticky top-0">
                               <tr>
                                 <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">物品</th>
                                 <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">尺寸</th>
                                 <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">重量</th>
                                 <th class="px-3 py-2 text-xs font-medium text-gray-500 text-left">位置</th>
                               </tr>
                             </thead>
                             <tbody class="bg-white divide-y divide-gray-100">
                               <tr v-for="(item, itemIndex) in box.items" :key="itemIndex" :class="getBoxColor(boxIndex) + ' bg-opacity-60 hover:bg-opacity-80'">
                                 <td class="px-3 py-2 text-sm">{{ getItemName(item.id) }}</td>
                                 <td class="px-3 py-2 text-sm text-gray-500">{{ item.length }}×{{ item.width }}×{{ item.height }}cm</td>
                                 <td class="px-3 py-2 text-sm text-gray-500">{{ item.weight }}kg</td>
                                 <td class="px-3 py-2 text-xs text-gray-500">
                                   <span v-if="item.position" class="font-mono">
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
                  <h3 class="text-sm font-semibold uppercase tracking-wider text-red-500 mb-3">未装箱物品</h3>

                  <div class="border border-red-200 rounded-lg overflow-hidden bg-red-50 shadow-sm">
                    <table class="min-w-full divide-y divide-red-200">
                      <thead class="bg-red-100">
                        <tr>
                          <th class="px-3 py-2 text-xs font-medium text-red-900 text-left">物品</th>
                          <th class="px-3 py-2 text-xs font-medium text-red-900 text-left">目的地</th>
                          <th class="px-3 py-2 text-xs font-medium text-red-900 text-left">尺寸/重量</th>
                          <th class="px-3 py-2 text-xs font-medium text-red-900 text-left">原因</th>
                        </tr>
                      </thead>
                      <tbody class="divide-y divide-red-200">
                        <tr v-for="(item, index) in solution.unpacked_items" :key="index" class="hover:bg-red-100">
                          <td class="px-3 py-2 text-sm">{{ getItemName(item.id) }}</td>
                          <td class="px-3 py-2 text-sm">{{ item.destination }}</td>
                          <td class="px-3 py-2 text-sm">{{ item.length }}×{{ item.width }}×{{ item.height }}cm, {{ item.weight }}kg</td>
                          <td class="px-3 py-2 text-sm text-red-600 font-medium">
                            {{ getUnpackedReason(item) }}
                          </td>
                        </tr>
                      </tbody>
                    </table>
                  </div>
                </div>
              </div>
            </div>

            <!-- Empty Result State -->
            <div v-else-if="!showResult && !isCalculating" class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 text-center h-full flex flex-col justify-center min-h-[500px]">
              <div class="py-16">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto text-gray-300 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                </svg>
                <h3 class="text-lg font-medium text-gray-600 mb-1">暂无装箱方案</h3>
                <p class="text-gray-400 max-w-sm mx-auto">请添加物品并计算以查看优化的装箱方案</p>
              </div>
            </div>

            <!-- Loading State -->
            <div v-else-if="isCalculating" class="bg-white rounded-xl shadow-sm p-5 border border-indigo-100 text-center h-full flex flex-col justify-center min-h-[500px]">
              <div class="py-16">
                <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-indigo-600 mx-auto"></div>
                <h3 class="text-lg font-medium text-gray-600 mt-4 mb-1">正在计算最优装箱方案</h3>
                <p class="text-gray-400 max-w-sm mx-auto">正在寻找最高效的物品装箱方式...</p>
              </div>
            </div>
          </div>

          <!-- Shipping Info Card -->
          <div v-if="showResult && solution" class="mt-5 bg-white rounded-xl shadow-sm p-5 border border-gray-100">
            <h3 class="text-sm font-semibold uppercase tracking-wider text-gray-500 mb-3">运输信息</h3>
            <div class="text-xs text-gray-500 space-y-1">
              <p>• 箱子重量包含纸板（厚度：0.6cm，重量：0.54 kg/m²）</p>
              <p>• 物品装箱时会在遵守目的地限制的前提下最小化空间浪费</p>
            </div>
          </div>
        </div>
      </div>
    </main>

    <footer class="bg-gray-50 border-t mt-8">
      <div class="container mx-auto px-4 py-4 text-center text-gray-500 text-sm">
        跨境电商装箱助手 &copy; {{ new Date().getFullYear() }}
      </div>
    </footer>
  </div>
</template>

<style>
@import "tailwindcss";
</style>