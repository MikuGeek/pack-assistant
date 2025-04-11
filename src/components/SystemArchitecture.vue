<template>
  <div class="w-full h-full flex flex-col">
    <h2 class="text-xl font-bold mb-4">System Architecture</h2>
    <div class="mermaid-container w-full h-full flex-grow" ref="mermaidContainer">
      <!-- Mermaid diagram will be rendered here -->
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import mermaid from 'mermaid';

const mermaidContainer = ref(null);

onMounted(() => {
  mermaid.initialize({
    startOnLoad: true,
    theme: 'default',
    securityLevel: 'loose',
  });

  const diagramDefinition = `
    graph TB
      subgraph "Frontend Modules"
        A[Item Management Module]
        B[Visualization Module]
        C[Results Analysis Module]
        D[User Interface Module]
      end

      subgraph "Backend Modules"
        E[Packing Algorithm Module]
        F[Data Storage Module]
      end

      D -- "Data Interaction" --> A
      D -- "Data Display" --> B
      D -- "Data Display" --> C
      A -- "Trigger Calculation" --> E
      E -- "Calculation Results" --> B
      E -- "Calculation Results" --> C
      A -- "Read/Write Data" --> F
      E -- "Read/Write Config/Cache" --> F
  `;

  // Render the mermaid diagram
  mermaid.render('mermaid', diagramDefinition).then(({ svg }) => {
    if (mermaidContainer.value) {
      mermaidContainer.value.innerHTML = svg;
    }
  });
});
</script>

<style scoped>
.mermaid-container {
  overflow: auto;
}
</style>