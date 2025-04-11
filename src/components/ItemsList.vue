<script setup lang="ts">
import { ref, computed } from 'vue';

// Define the Item interface
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

// Props definition
const props = defineProps<{
  items: Item[];
  standardItems: Record<string, { id: string; name: string; length: number; width: number; height: number; weight: number; destination: string }>;
}>();

// Define emits
const emit = defineEmits<{
  (e: 'remove-item', index: number): void;
  (e: 'remove-by-destination', destination: string): void;
  (e: 'calculate-packing'): void;
  (e: 'clear-all'): void;
}>();

// State
const showDetailedList = ref(false);
const isCalculating = ref(false);

// Computed values
const totalItems = computed(() => props.items.length);

// Calculate total weight of all items
const totalWeight = computed(() =>
  props.items.reduce((sum, item) => sum + item.weight, 0)
);

// Calculate total volume of all items
const totalVolume = computed(() =>
  props.items.reduce((sum, item) => sum + (item.length * item.width * item.height), 0)
);

// Organize items by destination
const itemsByDestination = computed(() => {
  const grouped: Record<string, Item[]> = {};

  props.items.forEach(item => {
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
  return props.standardItems[baseId]?.name || baseId;
}

// Function handlers
function removeItem(index: number) {
  emit('remove-item', index);
}

function removeItemsByDestination(destination: string) {
  emit('remove-by-destination', destination);
}

function calculatePacking() {
  emit('calculate-packing');
}

function clearAll() {
  emit('clear-all');
}
</script>

<template>
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
                    @click="removeItemsByDestination(destination)"
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
</template>