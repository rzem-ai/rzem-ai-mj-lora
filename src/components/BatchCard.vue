<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
    <div class="flex justify-between items-start mb-4">
      <div class="flex-1">
        <input
          v-model="localBatch.batch_name"
          @blur="emitUpdate"
          class="text-lg font-bold bg-transparent border-b border-transparent hover:border-gray-300 dark:hover:border-gray-600 focus:border-blue-500 outline-none text-gray-900 dark:text-white w-full"
        />
        <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
          Batch {{ localBatch.batch_number }}
        </p>
      </div>

      <div class="flex space-x-2">
        <button
          @click="$emit('duplicate')"
          class="p-2 text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
          title="Duplicate batch"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
            />
          </svg>
        </button>
        <button
          @click="$emit('remove')"
          class="p-2 text-red-600 dark:text-red-400 hover:bg-red-100 dark:hover:bg-red-900/20 rounded"
          title="Remove batch"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
            />
          </svg>
        </button>
      </div>
    </div>

    <div class="grid md:grid-cols-3 gap-4 mb-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Category
        </label>
        <input
          v-model="localBatch.category"
          @blur="emitUpdate"
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
        />
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Priority
        </label>
        <select
          v-model="localBatch.priority"
          @change="emitUpdate"
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
        >
          <option value="high">High</option>
          <option value="medium">Medium</option>
          <option value="low">Low</option>
        </select>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Image Count
        </label>
        <div class="flex items-center space-x-2">
          <span
            :class="{
              'text-green-600 dark:text-green-400': validation.calculatedCount === 40,
              'text-red-600 dark:text-red-400': validation.calculatedCount !== 40,
            }"
            class="text-2xl font-bold"
          >
            {{ validation.calculatedCount }}
          </span>
          <span class="text-sm text-gray-600 dark:text-gray-400">/ 40</span>
        </div>
      </div>
    </div>

    <div class="mb-4">
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Prompt
      </label>
      <textarea
        v-model="localBatch.prompt"
        @blur="emitUpdate"
        rows="3"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white font-mono text-sm"
      />
    </div>

    <div class="mb-4">
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Notes (Optional)
      </label>
      <textarea
        v-model="localBatch.notes"
        @blur="emitUpdate"
        rows="2"
        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
      />
    </div>

    <!-- Validation Messages -->
    <div v-if="!validation.isValid || validation.warnings.length > 0" class="space-y-2">
      <div
        v-for="error in validation.errors"
        :key="error"
        class="text-sm text-red-600 dark:text-red-400 flex items-start"
      >
        <svg class="w-4 h-4 mr-1 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
            clip-rule="evenodd"
          />
        </svg>
        {{ error }}
      </div>
      <div
        v-for="warning in validation.warnings"
        :key="warning"
        class="text-sm text-yellow-600 dark:text-yellow-400 flex items-start"
      >
        <svg class="w-4 h-4 mr-1 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
          <path
            fill-rule="evenodd"
            d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
            clip-rule="evenodd"
          />
        </svg>
        {{ warning }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { PermutationBatch } from '../types/schema';
import { validateBatch } from '../utils/validation';

const props = defineProps<{
  batch: PermutationBatch;
  srefCode: string;
}>();

const emit = defineEmits<{
  (e: 'update', batch: PermutationBatch): void;
  (e: 'duplicate'): void;
  (e: 'remove'): void;
}>();

const localBatch = ref<PermutationBatch>({ ...props.batch });

// Watch for external changes
watch(() => props.batch, (newBatch) => {
  localBatch.value = { ...newBatch };
}, { deep: true });

const validation = computed(() => validateBatch(localBatch.value, props.srefCode));

const emitUpdate = () => {
  emit('update', { ...localBatch.value });
};
</script>
