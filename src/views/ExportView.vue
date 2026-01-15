<template>
  <div class="max-w-4xl mx-auto">
    <div v-if="specification" class="space-y-6">
      <!-- Summary Card -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Dataset Summary</h2>

        <div class="grid gap-4 mb-6 md:grid-cols-3">
          <div class="p-4 rounded bg-blue-50 dark:bg-blue-900/20">
            <p class="text-sm text-blue-600 dark:text-blue-400">Total Images</p>
            <p class="text-3xl font-bold text-blue-900 dark:text-blue-300">
              {{ summary.totalImages }}
            </p>
          </div>
          <div class="p-4 rounded bg-green-50 dark:bg-green-900/20">
            <p class="text-sm text-green-600 dark:text-green-400">Total Batches</p>
            <p class="text-3xl font-bold text-green-900 dark:text-green-300">
              {{ summary.totalBatches }}
            </p>
          </div>
          <div class="p-4 rounded bg-purple-50 dark:bg-purple-900/20">
            <p class="text-sm text-purple-600 dark:text-purple-400">Categories</p>
            <p class="text-3xl font-bold text-purple-900 dark:text-purple-300">
              {{ summary.categories }}
            </p>
          </div>
        </div>

        <!-- Validation Status -->
        <div
          :class="{
            'bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800': validation.isValid,
            'bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800': !validation.isValid,
          }"
          class="p-4 border-2 rounded-lg"
        >
          <div class="flex items-center mb-2">
            <svg v-if="validation.isValid" class="w-6 h-6 mr-2 text-green-600 dark:text-green-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
            </svg>
            <svg v-else class="w-6 h-6 mr-2 text-red-600 dark:text-red-400" fill="currentColor" viewBox="0 0 20 20">
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                clip-rule="evenodd"
              />
            </svg>
            <h3
              :class="{
                'text-green-900 dark:text-green-300': validation.isValid,
                'text-red-900 dark:text-red-300': !validation.isValid,
              }"
              class="text-lg font-semibold"
            >
              {{ validation.isValid ? 'Dataset is valid!' : 'Dataset has errors' }}
            </h3>
          </div>

          <div v-if="validation.errors.length > 0" class="mb-2 space-y-1">
            <p v-for="error in validation.errors" :key="error" class="text-sm text-red-700 dark:text-red-400">• {{ error }}</p>
          </div>

          <div v-if="validation.warnings.length > 0" class="space-y-1">
            <p v-for="warning in validation.warnings" :key="warning" class="text-sm text-yellow-700 dark:text-yellow-400">⚠ {{ warning }}</p>
          </div>
        </div>
      </div>

      <!-- Export Panel -->
      <ExportPanel />

      <!-- Project Actions -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h3 class="mb-4 text-lg font-semibold text-gray-900 dark:text-white">Project</h3>
        <div class="flex space-x-4">
          <button
            @click="saveProject"
            :disabled="isSaving"
            class="flex-1 px-6 py-3 font-medium text-white transition-colors bg-purple-600 rounded-lg hover:bg-purple-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ isSaving ? 'Saving...' : 'Save Project' }}
          </button>
          <button @click="loadProject" class="flex-1 px-6 py-3 font-medium text-white transition-colors bg-gray-600 rounded-lg hover:bg-gray-700">Load Project</button>
        </div>
      </div>

      <!-- Navigation -->
      <div class="flex justify-between">
        <button @click="goBack" class="px-6 py-3 font-medium text-gray-900 transition-colors bg-gray-300 rounded-lg dark:bg-gray-700 dark:text-white hover:bg-gray-400 dark:hover:bg-gray-600">
          Back to Batches
        </button>
        <button @click="startOver" class="px-6 py-3 font-medium text-white transition-colors bg-red-600 rounded-lg hover:bg-red-700">Start Over</button>
      </div>
    </div>

    <div v-else class="py-12 text-center text-gray-600 dark:text-gray-400">No specification available. Please complete the analysis first.</div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import { useProjectStore } from '../stores/project';
import { open, save } from '@tauri-apps/plugin-dialog';
import ExportPanel from '../components/ExportPanel.vue';
import { validateDatasetSpecification, generateDatasetSummary } from '../utils/validation';

const store = useProjectStore();
const router = useRouter();

const isSaving = ref(false);

const specification = computed(() => store.specification);
const validation = computed(() => (specification.value ? validateDatasetSpecification(specification.value) : { isValid: false, errors: [], warnings: [] }));
const summary = computed(() => (specification.value ? generateDatasetSummary(specification.value) : { totalImages: 0, totalBatches: 0, categories: 0 }));

const saveProject = async () => {
  isSaving.value = true;
  try {
    const filePath = await save({
      filters: [
        {
          name: 'LoRA Project',
          extensions: ['lora-project', 'json'],
        },
      ],
      defaultPath: `lora-${store.srefCode}.lora-project`,
    });

    if (filePath) {
      await store.saveProject(filePath);
      alert('Project saved successfully!');
    }
  } catch (e) {
    alert(`Failed to save project: ${e}`);
  } finally {
    isSaving.value = false;
  }
};

const loadProject = async () => {
  try {
    const filePath = await open({
      filters: [
        {
          name: 'LoRA Project',
          extensions: ['lora-project', 'json'],
        },
      ],
      multiple: false,
    });

    if (filePath && typeof filePath === 'string') {
      await store.loadProject(filePath);
      alert('Project loaded successfully!');
    }
  } catch (e) {
    alert(`Failed to load project: ${e}`);
  }
};

const goBack = () => {
  router.push('/batches');
};

const startOver = () => {
  if (confirm('Are you sure you want to start over? All unsaved changes will be lost.')) {
    store.reset();
    router.push('/');
  }
};
</script>
