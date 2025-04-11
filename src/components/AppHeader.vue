<script setup lang="ts">
// Header component
const emit = defineEmits(['reset']);

function handleReset() {
  emit('reset');
}
</script>

<template>
  <header class="bg-indigo-700 text-white shadow-lg">
    <div class="container mx-auto px-4 py-5">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold">装箱助手</h1>
          <p class="text-indigo-100 text-sm">优化跨境电商物流装箱</p>
        </div>
        <div class="flex items-center space-x-4">
          <button
            @click="handleReset"
            class="px-3 py-1.5 bg-red-600 hover:bg-red-700 rounded text-sm font-medium transition-colors"
          >
            重置
          </button>
          <div class="hidden md:block">
            <span class="px-3 py-1 bg-indigo-800 rounded-full text-xs">由 Tauri + Vue 驱动</span>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>