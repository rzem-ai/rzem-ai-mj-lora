<template>
  <div class="max-w-4xl mx-auto space-y-6">
    <!-- Header -->
    <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Settings</h1>
      <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
        Configure analysis mode and offline model preferences
      </p>
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
      <div class="flex items-center justify-center py-12">
        <div class="text-gray-600 dark:text-gray-400">Loading settings...</div>
      </div>
    </div>

    <!-- Settings Content -->
    <div v-else-if="localSettings" class="space-y-6">
      <!-- Analysis Mode Selection -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Analysis Mode</h2>
        <p class="mb-6 text-sm text-gray-600 dark:text-gray-400">
          Choose how style analysis is performed
        </p>

        <div class="space-y-3">
          <label class="flex items-start p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50"
                 :class="localSettings.analysis_mode === 'CloudAPI' ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
            <input
              type="radio"
              value="CloudAPI"
              v-model="localSettings.analysis_mode"
              class="mt-1 mr-3"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Cloud API</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Use Claude API for fast, high-quality analysis (requires API key)
              </div>
            </div>
          </label>

          <label class="flex items-start p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50"
                 :class="localSettings.analysis_mode === 'Offline' ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
            <input
              type="radio"
              value="Offline"
              v-model="localSettings.analysis_mode"
              class="mt-1 mr-3"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Offline Mode</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Use local Qwen2-VL model (private, no API costs, slower)
              </div>
            </div>
          </label>

          <label class="flex items-start p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50"
                 :class="localSettings.analysis_mode === 'Auto' ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'">
            <input
              type="radio"
              value="Auto"
              v-model="localSettings.analysis_mode"
              class="mt-1 mr-3"
            />
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Auto (Recommended)</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Use API if available, otherwise offline mode
              </div>
            </div>
          </label>
        </div>
      </div>

      <!-- Model Variant Selection -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Offline Model Variant</h2>
        <p class="mb-6 text-sm text-gray-600 dark:text-gray-400">
          Choose which Qwen2-VL model to use for offline analysis
        </p>

        <div class="grid gap-4 md:grid-cols-3">
          <!-- Qwen2-VL-2B -->
          <div
            class="p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50"
            :class="localSettings.offline_model_variant === 'Qwen2VL2B' ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'"
            @click="localSettings.offline_model_variant = 'Qwen2VL2B'"
          >
            <div class="flex items-start justify-between mb-3">
              <input
                type="radio"
                value="Qwen2VL2B"
                v-model="localSettings.offline_model_variant"
                class="mt-1"
              />
              <span class="px-2 py-1 text-xs font-medium text-green-700 bg-green-100 rounded dark:bg-green-900/20 dark:text-green-400">
                Good
              </span>
            </div>
            <div class="mb-2 font-bold text-gray-900 dark:text-white">Qwen2-VL-2B</div>
            <div class="mb-3 text-sm text-gray-600 dark:text-gray-400">
              Fast inference, good quality
            </div>
            <div class="space-y-1 text-xs text-gray-500 dark:text-gray-500">
              <div>• Size: 4.5 GB</div>
              <div>• RAM: 4 GB required</div>
              <div>• Speed: 10-20s</div>
            </div>
          </div>

          <!-- Qwen2-VL-7B -->
          <div
            class="p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50"
            :class="localSettings.offline_model_variant === 'Qwen2VL7B' ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'"
            @click="localSettings.offline_model_variant = 'Qwen2VL7B'"
          >
            <div class="flex items-start justify-between mb-3">
              <input
                type="radio"
                value="Qwen2VL7B"
                v-model="localSettings.offline_model_variant"
                class="mt-1"
              />
              <span class="px-2 py-1 text-xs font-medium text-blue-700 bg-blue-100 rounded dark:bg-blue-900/20 dark:text-blue-400">
                Better
              </span>
            </div>
            <div class="mb-2 font-bold text-gray-900 dark:text-white">Qwen2-VL-7B</div>
            <div class="mb-3 text-sm text-gray-600 dark:text-gray-400">
              Moderate speed, better quality
            </div>
            <div class="space-y-1 text-xs text-gray-500 dark:text-gray-500">
              <div>• Size: 15 GB</div>
              <div>• RAM: 12 GB required</div>
              <div>• Speed: 30-60s</div>
            </div>
          </div>

          <!-- Qwen2-VL-72B -->
          <div
            class="p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50"
            :class="localSettings.offline_model_variant === 'Qwen2VL72B' ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : 'border-gray-300 dark:border-gray-600'"
            @click="localSettings.offline_model_variant = 'Qwen2VL72B'"
          >
            <div class="flex items-start justify-between mb-3">
              <input
                type="radio"
                value="Qwen2VL72B"
                v-model="localSettings.offline_model_variant"
                class="mt-1"
              />
              <span class="px-2 py-1 text-xs font-medium text-purple-700 bg-purple-100 rounded dark:bg-purple-900/20 dark:text-purple-400">
                Best
              </span>
            </div>
            <div class="mb-2 font-bold text-gray-900 dark:text-white">Qwen2-VL-72B</div>
            <div class="mb-3 text-sm text-gray-600 dark:text-gray-400">
              Slow inference, highest quality
            </div>
            <div class="space-y-1 text-xs text-gray-500 dark:text-gray-500">
              <div>• Size: 146 GB</div>
              <div>• RAM: 80 GB required</div>
              <div>• Speed: 2-5 min</div>
            </div>
          </div>
        </div>

        <!-- Model Status -->
        <div class="p-4 mt-4 bg-gray-100 rounded-lg dark:bg-gray-700/50">
          <div class="flex items-center justify-between">
            <div>
              <div class="font-medium text-gray-900 dark:text-white">
                Model Status: {{ modelStatusText }}
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                {{ modelStatusMessage }}
              </div>
            </div>
            <button
              v-if="modelStatus?.status === 'NotDownloaded'"
              @click="downloadModel"
              :disabled="isDownloading"
              class="px-4 py-2 font-medium text-white transition-colors bg-blue-600 rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ isDownloading ? 'Downloading...' : 'Download Model' }}
            </button>
          </div>
        </div>
      </div>

      <!-- Advanced Options -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Advanced Options</h2>

        <div class="space-y-4">
          <label class="flex items-center justify-between p-4 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50">
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Auto Fallback</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Automatically use offline mode if API fails
              </div>
            </div>
            <input
              type="checkbox"
              v-model="localSettings.auto_fallback"
              class="w-5 h-5"
            />
          </label>

          <label class="flex items-center justify-between p-4 rounded-lg cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700/50">
            <div>
              <div class="font-medium text-gray-900 dark:text-white">Keep Model Loaded</div>
              <div class="text-sm text-gray-600 dark:text-gray-400">
                Keep model in memory between analyses (faster, uses more RAM)
              </div>
            </div>
            <input
              type="checkbox"
              v-model="localSettings.keep_model_loaded"
              class="w-5 h-5"
            />
          </label>
        </div>
      </div>

      <!-- Cache Management -->
      <div class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">Cache Management</h2>

        <div class="flex items-center justify-between p-4 bg-gray-100 rounded-lg dark:bg-gray-700/50">
          <div>
            <div class="font-medium text-gray-900 dark:text-white">Model Cache</div>
            <div class="text-sm text-gray-600 dark:text-gray-400">
              Clear downloaded models to free disk space
            </div>
          </div>
          <button
            @click="clearCache"
            :disabled="isClearing"
            class="px-4 py-2 font-medium text-white transition-colors bg-red-600 rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ isClearing ? 'Clearing...' : 'Clear Cache' }}
          </button>
        </div>
      </div>

      <!-- Save/Cancel Actions -->
      <div class="flex justify-between gap-4">
        <button
          @click="goBack"
          class="px-6 py-3 font-medium text-gray-900 transition-colors bg-gray-300 rounded-lg dark:bg-gray-700 dark:text-white hover:bg-gray-400 dark:hover:bg-gray-600"
        >
          Cancel
        </button>
        <button
          @click="saveAndGoBack"
          :disabled="isSaving"
          class="px-6 py-3 font-medium text-white transition-colors bg-blue-600 rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {{ isSaving ? 'Saving...' : 'Save Settings' }}
        </button>
      </div>
    </div>

    <!-- Error State -->
    <div v-else class="p-6 bg-white rounded-lg shadow dark:bg-gray-800">
      <div class="py-12 text-center">
        <div class="mb-4 text-red-600 dark:text-red-400">Failed to load settings</div>
        <button
          @click="loadSettingsData"
          class="px-4 py-2 font-medium text-white transition-colors bg-blue-600 rounded-lg hover:bg-blue-700"
        >
          Retry
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore, type AppSettings, type ModelVariant } from '../stores/project';

const store = useProjectStore();
const router = useRouter();

const localSettings = ref<AppSettings | null>(null);
const isLoading = ref(true);
const isSaving = ref(false);
const isDownloading = ref(false);
const isClearing = ref(false);
const modelStatus = ref<any>(null);

const modelStatusText = computed(() => {
  if (!modelStatus.value) return 'Unknown';

  const status = modelStatus.value.status;
  if (status === 'NotDownloaded') return 'Not Downloaded';
  if (status === 'Ready') return 'Ready';
  if (status === 'Error') return 'Error';
  if (status === 'Downloading') {
    const progress = modelStatus.value.progress_percent || 0;
    return `Downloading (${progress}%)`;
  }
  return 'Unknown';
});

const modelStatusMessage = computed(() => {
  if (!modelStatus.value) return '';

  const status = modelStatus.value.status;
  if (status === 'NotDownloaded') return 'Model needs to be downloaded before use';
  if (status === 'Ready') return 'Model is ready for offline analysis';
  if (status === 'Error') return modelStatus.value.message || 'An error occurred';
  if (status === 'Downloading') return 'Downloading model files...';
  return '';
});

async function loadSettingsData() {
  isLoading.value = true;
  try {
    await store.loadSettings();
    localSettings.value = JSON.parse(JSON.stringify(store.settings));
    await checkModelStatus();
  } catch (error) {
    console.error('Failed to load settings:', error);
  } finally {
    isLoading.value = false;
  }
}

async function checkModelStatus() {
  if (!localSettings.value) return;

  try {
    const status = await invoke('get_model_status', {
      variant: localSettings.value.offline_model_variant
    });
    modelStatus.value = status;
  } catch (error) {
    console.error('Failed to check model status:', error);
  }
}

async function downloadModel() {
  if (!localSettings.value) return;

  isDownloading.value = true;
  try {
    await invoke('download_model', {
      variant: localSettings.value.offline_model_variant
    });
    await checkModelStatus();
  } catch (error) {
    console.error('Failed to download model:', error);
    alert(`Failed to download model: ${error}`);
  } finally {
    isDownloading.value = false;
  }
}

async function clearCache() {
  if (!confirm('Are you sure you want to clear the model cache? This will delete all downloaded models.')) {
    return;
  }

  isClearing.value = true;
  try {
    const bytesFreed = await invoke<number>('clear_model_cache');
    const mbFreed = (bytesFreed / 1024 / 1024).toFixed(2);
    alert(`Successfully cleared ${mbFreed} MB from cache`);
    await checkModelStatus();
  } catch (error) {
    console.error('Failed to clear cache:', error);
    alert(`Failed to clear cache: ${error}`);
  } finally {
    isClearing.value = false;
  }
}

async function saveAndGoBack() {
  if (!localSettings.value) return;

  isSaving.value = true;
  try {
    await store.saveSettings(localSettings.value);
    router.push('/');
  } catch (error) {
    console.error('Failed to save settings:', error);
    alert(`Failed to save settings: ${error}`);
  } finally {
    isSaving.value = false;
  }
}

function goBack() {
  router.push('/');
}

onMounted(() => {
  loadSettingsData();
});
</script>
