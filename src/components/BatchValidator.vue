<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Validation Status</h3>
      <div
        :class="{
          'bg-green-100 dark:bg-green-900/20 text-green-800 dark:text-green-400': validation.isValid,
          'bg-red-100 dark:bg-red-900/20 text-red-800 dark:text-red-400': !validation.isValid,
        }"
        class="px-3 py-1 rounded-full text-sm font-medium"
      >
        {{ validation.isValid ? '✓ Valid' : '✗ Invalid' }}
      </div>
    </div>

    <div class="grid md:grid-cols-4 gap-4">
      <div class="bg-gray-50 dark:bg-gray-700 p-3 rounded">
        <p class="text-xs text-gray-600 dark:text-gray-400">Total Batches</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">
          {{ summary.totalBatches }}
        </p>
      </div>
      <div class="bg-gray-50 dark:bg-gray-700 p-3 rounded">
        <p class="text-xs text-gray-600 dark:text-gray-400">Total Images</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">
          {{ summary.totalImages }}
        </p>
      </div>
      <div class="bg-gray-50 dark:bg-gray-700 p-3 rounded">
        <p class="text-xs text-gray-600 dark:text-gray-400">High Priority</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">
          {{ summary.highPriorityBatches }}
        </p>
      </div>
      <div class="bg-gray-50 dark:bg-gray-700 p-3 rounded">
        <p class="text-xs text-gray-600 dark:text-gray-400">Categories</p>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">
          {{ summary.categories }}
        </p>
      </div>
    </div>

    <div v-if="validation.errors.length > 0" class="space-y-2">
      <h4 class="text-sm font-medium text-red-700 dark:text-red-400">Errors:</h4>
      <div
        v-for="error in validation.errors"
        :key="error"
        class="text-sm text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/10 p-2 rounded"
      >
        {{ error }}
      </div>
    </div>

    <div v-if="validation.warnings.length > 0" class="space-y-2">
      <h4 class="text-sm font-medium text-yellow-700 dark:text-yellow-400">Warnings:</h4>
      <div
        v-for="warning in validation.warnings"
        :key="warning"
        class="text-sm text-yellow-600 dark:text-yellow-400 bg-yellow-50 dark:bg-yellow-900/10 p-2 rounded"
      >
        {{ warning }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { DatasetSpecification } from '../types/schema';
import { validateDatasetSpecification, generateDatasetSummary } from '../utils/validation';

const props = defineProps<{
  specification: DatasetSpecification;
}>();

const validation = computed(() => validateDatasetSpecification(props.specification));
const summary = computed(() => generateDatasetSummary(props.specification));
</script>
