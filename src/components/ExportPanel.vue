<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Export Dataset</h3>

    <div class="space-y-4">
      <!-- Format Selection -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Export Format
        </label>
        <div class="flex space-x-4">
          <button
            @click="selectedFormat = 'json'"
            :class="{
              'bg-blue-600 text-white': selectedFormat === 'json',
              'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white': selectedFormat !== 'json',
            }"
            class="flex-1 px-4 py-3 rounded-lg font-medium transition-colors"
          >
            <div class="text-left">
              <p class="font-bold">JSON</p>
              <p class="text-xs opacity-90">Structured data format</p>
            </div>
          </button>
          <button
            @click="selectedFormat = 'markdown'"
            :class="{
              'bg-blue-600 text-white': selectedFormat === 'markdown',
              'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white': selectedFormat !== 'markdown',
            }"
            class="flex-1 px-4 py-3 rounded-lg font-medium transition-colors"
          >
            <div class="text-left">
              <p class="font-bold">Markdown</p>
              <p class="text-xs opacity-90">Human-readable format</p>
            </div>
          </button>
        </div>
      </div>

      <!-- Preview -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Preview
        </label>
        <div class="bg-gray-50 dark:bg-gray-900 rounded-lg p-4 max-h-64 overflow-auto">
          <pre class="text-xs text-gray-900 dark:text-gray-100 whitespace-pre-wrap">{{ preview }}</pre>
        </div>
      </div>

      <!-- Export Buttons -->
      <div class="flex space-x-4">
        <button
          @click="exportToFile"
          :disabled="isExporting"
          class="flex-1 px-6 py-3 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {{ isExporting ? 'Exporting...' : `Export as ${selectedFormat.toUpperCase()}` }}
        </button>
        <button
          @click="copyToClipboard"
          class="px-6 py-3 bg-gray-600 text-white rounded-lg font-medium hover:bg-gray-700 transition-colors"
        >
          Copy to Clipboard
        </button>
      </div>

      <div v-if="exportSuccess" class="p-4 bg-green-100 dark:bg-green-900/20 text-green-800 dark:text-green-400 rounded-lg">
        ✓ {{ exportSuccess }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useProjectStore } from '../stores/project';
import { save } from '@tauri-apps/plugin-dialog';
import { exportAsJSON, exportAsMarkdown } from '../utils/export';

const store = useProjectStore();

const selectedFormat = ref<'json' | 'markdown'>('json');
const isExporting = ref(false);
const exportSuccess = ref<string | null>(null);

const preview = computed(() => {
  if (!store.specification) return '';

  if (selectedFormat.value === 'json') {
    const json = exportAsJSON(store.specification);
    // Show first 1000 characters for preview
    return json.length > 1000 ? json.substring(0, 1000) + '\n\n... (truncated)' : json;
  } else {
    const md = exportAsMarkdown(store.specification);
    // Show first 1000 characters for preview
    return md.length > 1000 ? md.substring(0, 1000) + '\n\n... (truncated)' : md;
  }
});

const exportToFile = async () => {
  if (!store.specification) return;

  isExporting.value = true;
  exportSuccess.value = null;

  try {
    const extension = selectedFormat.value === 'json' ? 'json' : 'md';
    const filePath = await save({
      filters: [
        {
          name: selectedFormat.value.toUpperCase(),
          extensions: [extension],
        },
      ],
      defaultPath: `lora-${store.srefCode}.${extension}`,
    });

    if (filePath) {
      if (selectedFormat.value === 'json') {
        await store.exportJSON(filePath);
      } else {
        await store.exportMarkdown(filePath);
      }
      exportSuccess.value = `Exported successfully to ${filePath}`;

      // Clear success message after 3 seconds
      setTimeout(() => {
        exportSuccess.value = null;
      }, 3000);
    }
  } catch (e) {
    alert(`Failed to export: ${e}`);
  } finally {
    isExporting.value = false;
  }
};

const copyToClipboard = async () => {
  if (!store.specification) return;

  const content = selectedFormat.value === 'json'
    ? exportAsJSON(store.specification)
    : exportAsMarkdown(store.specification);

  try {
    await navigator.clipboard.writeText(content);
    exportSuccess.value = 'Copied to clipboard!';

    // Clear success message after 3 seconds
    setTimeout(() => {
      exportSuccess.value = null;
    }, 3000);
  } catch (e) {
    alert(`Failed to copy: ${e}`);
  }
};
</script>
