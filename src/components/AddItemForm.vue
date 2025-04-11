<script setup lang="ts">
import { reactive, computed } from 'vue';

// Define props and emits
const props = defineProps<{
  standardItems: Record<string, { id: string; name: string; length: number; width: number; height: number; weight: number; destination: string }>
}>();

const emit = defineEmits<{
  (e: 'add-items', itemId: string, quantity: number): void
}>();

// Form state for adding items
const newItemForm = reactive({
  combinedSelection: "",
  quantity: 1,
});

// Validation state
const errors = reactive<Record<string, string>>({});

// Get all available item options
const availableItemOptions = computed(() =>
  Object.values(props.standardItems).map(item => ({
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

  Object.assign(errors, newErrors);

  // If there are errors, don't add items
  if (Object.keys(newErrors).length > 0) return;

  // Emit the event to add items
  emit('add-items', newItemForm.combinedSelection, newItemForm.quantity);

  // Reset quantity to 1 after adding
  newItemForm.quantity = 1;
}
</script>

<template>
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
</template>