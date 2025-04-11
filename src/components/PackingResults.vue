<script setup lang="ts">
import { ref, computed } from 'vue';

// Define interfaces
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

// Props
const props = defineProps<{
  solution: PackingSolution | null;
  isCalculating: boolean;
  showResult: boolean;
  standardItems: Record<string, { id: string; name: string; length: number; width: number; height: number; weight: number; destination: string }>;
}>();

// Emit events
const emit = defineEmits<{
  (e: 'select-box', index: number): void;
}>();

// Local state
const selectedBoxIndex = ref<number | null>(null);

// Computed values
const selectedBox = computed(() => {
  if (props.solution && selectedBoxIndex.value !== null && props.solution.boxes[selectedBoxIndex.value]) {
    return props.solution.boxes[selectedBoxIndex.value];
  }
  return null;
});

// Function to select a box for visualization
function selectBoxForVisualization(index: number) {
  selectedBoxIndex.value = index;
  emit('select-box', index);
}

// Calculate box space utilization
function calculateUtilization(box: PackedBox): number {
  const boxVolume = box.length * box.width * box.height;
  const itemsVolume = box.items.reduce((sum, item) => sum + (item.length * item.width * item.height), 0);
  return (itemsVolume / boxVolume) * 100;
}

// Helper function to get box color
function getBoxColor(index: number): string {
  const colors = ["bg-blue-200", "bg-green-200", "bg-yellow-200", "bg-red-200", "bg-purple-200", "bg-pink-200"];
  return colors[index % colors.length];
}

// Get item name from standard item
function getItemName(itemId: string): string {
  const baseId = itemId.split('-').slice(0, -1).join('-');
  return props.standardItems[baseId]?.name || baseId;
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

</script>

<template>
  <div class="w-full">
    <!-- Results Section -->
    <div v-if="showResult && solution" class="space-y-5">
      <!-- Packing Solution Details Card -->
      <div class="bg-white rounded-xl shadow-sm p-5 border border-gray-100 w-full">
        <h2 class="text-lg font-semibold text-gray-800 mb-4 flex items-center">
          <div class="flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 text-emerald-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            优化装箱方案
          </div>
          <!-- Download CSV Button REMOVED FROM HERE -->
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

      <!-- Shipping Info Card -->
      <div class="mt-5 bg-white rounded-xl shadow-sm p-5 border border-gray-100">
        <h3 class="text-sm font-semibold uppercase tracking-wider text-gray-500 mb-3">运输信息</h3>
        <div class="text-xs text-gray-500 space-y-1">
          <p>• 箱子重量包含纸板（厚度：0.6cm，重量：0.54 kg/m²）</p>
          <p>• 物品装箱时会在遵守目的地限制的前提下最小化空间浪费</p>
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
</template>