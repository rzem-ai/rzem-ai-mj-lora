<template>
  <div class="mx-auto max-w-7xl">
    <div v-if="specification" class="space-y-6">
      <!-- Header with Actions -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-xl font-bold text-gray-900 dark:text-white">Permutation Batches</h2>
          <button @click="addBatch" class="px-4 py-2 font-medium text-white transition-colors bg-green-600 rounded-lg hover:bg-green-700">+ Add Batch</button>
        </div>

        <!-- Validation Summary -->
        <BatchValidator :specification="specification" />
      </div>

      <!-- Batch Cards Grid -->
      <div class="grid gap-6">
        <BatchCard
          v-for="(batch, index) in specification.permutation_batches"
          :key="batch.batch_number"
          :batch="batch"
          :sref-code="specification.sref_code"
          @update="(updated) => updateBatch(index, updated)"
          @duplicate="() => duplicateBatch(index)"
          @remove="() => removeBatch(index)"
        />
      </div>

      <!-- Navigation -->
      <div class="flex justify-between">
        <button @click="goBack" class="px-6 py-3 font-medium text-gray-900 transition-colors bg-gray-300 rounded-lg dark:bg-gray-700 dark:text-white hover:bg-gray-400 dark:hover:bg-gray-600">
          Back
        </button>
        <button @click="goToExport" class="px-6 py-3 font-medium text-white transition-colors bg-blue-600 rounded-lg hover:bg-blue-700">Export</button>
      </div>
    </div>

    <div v-else class="py-12 text-center text-gray-600 dark:text-gray-400">No specification available. Please complete the analysis first.</div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import { useProjectStore } from '../stores/project';
import BatchCard from '../components/BatchCard.vue';
import BatchValidator from '../components/BatchValidator.vue';
import type { PermutationBatch } from '../types/schema';

const store = useProjectStore();
const router = useRouter();

const specification = computed(() => store.specification);

const updateBatch = (index: number, batch: PermutationBatch) => {
  store.updateBatch(index, batch);
};

const addBatch = () => {
  store.addBatch();
};

const duplicateBatch = (index: number) => {
  store.duplicateBatch(index);
};

const removeBatch = (index: number) => {
  if (confirm('Are you sure you want to remove this batch?')) {
    store.removeBatch(index);
  }
};

const goBack = () => {
  router.push('/analysis');
};

const goToExport = () => {
  store.goToStep('export');
  router.push('/export');
};
</script>
