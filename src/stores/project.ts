import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type {
  ProjectData,
  DatasetSpecification,
  PermutationBatch,
} from '../types/schema';
import { exportAsJSON, exportAsMarkdown } from '../utils/export';

export type Step = 'upload' | 'analysis' | 'batches' | 'export';

// Settings types (matches Rust backend)
export type AnalysisMode = 'CloudAPI' | 'Offline' | 'Auto';
export type ModelVariant = 'Qwen2VL2B' | 'Qwen2VL7B' | 'Qwen2VL72B';

export interface AppSettings {
  analysis_mode: AnalysisMode;
  offline_model_variant: ModelVariant;
  model_cache_dir: string | null;
  auto_fallback: boolean;
  keep_model_loaded: boolean;
}

export interface AnalysisResult {
  data: string;
  mode_used: string; // "cloud" or "offline"
  fallback_used: boolean;
}

export const useProjectStore = defineStore('project', () => {
  // State
  const imagePaths = ref<string[]>([]);
  const srefCode = ref('');
  const specification = ref<DatasetSpecification | null>(null);
  const currentStep = ref<Step>('upload');
  const isDirty = ref(false);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const statusMessage = ref<string | null>(null);
  const settings = ref<AppSettings | null>(null);
  const lastModeUsed = ref<string | null>(null);
  const lastFallbackUsed = ref(false);

  // Computed
  const hasImages = computed(() => imagePaths.value.length >= 3);
  const hasSrefCode = computed(() => srefCode.value.length >= 3);
  const canAnalyze = computed(() => hasImages.value && hasSrefCode.value);
  const hasSpecification = computed(() => specification.value !== null);

  // Actions
  function setImages(paths: string[]) {
    imagePaths.value = paths;
    isDirty.value = true;
  }

  function addImage(path: string) {
    if (!imagePaths.value.includes(path)) {
      imagePaths.value.push(path);
      isDirty.value = true;
    }
  }

  function removeImage(index: number) {
    imagePaths.value.splice(index, 1);
    isDirty.value = true;
  }

  function setSrefCode(code: string) {
    srefCode.value = code;
    isDirty.value = true;
  }

  async function analyzeStyle() {
    if (!canAnalyze.value) {
      throw new Error('Need at least 3 images and SREF code');
    }

    isLoading.value = true;
    error.value = null;
    statusMessage.value = null;

    try {
      statusMessage.value = `Reading ${imagePaths.value.length} image${imagePaths.value.length > 1 ? 's' : ''}...`;

      // Small delay to show the first status
      await new Promise(resolve => setTimeout(resolve, 100));

      statusMessage.value = 'Encoding images to base64...';
      await new Promise(resolve => setTimeout(resolve, 100));

      // Determine which mode will be used based on settings
      const currentSettings = settings.value;
      const modeHint = currentSettings?.analysis_mode || 'Auto';

      if (modeHint === 'Offline') {
        statusMessage.value = 'Loading Qwen2-VL model (first time: 10-15s)...';
      } else {
        statusMessage.value = 'Sending request to Claude API...';
      }
      await new Promise(resolve => setTimeout(resolve, 100));

      statusMessage.value = 'Analyzing style characteristics...';

      const result = await invoke<AnalysisResult>('analyze_style', {
        imagePaths: imagePaths.value,
        srefCode: String(srefCode.value),
      });

      // Store which mode was actually used
      lastModeUsed.value = result.mode_used;
      lastFallbackUsed.value = result.fallback_used;

      statusMessage.value = 'Processing response...';
      await new Promise(resolve => setTimeout(resolve, 100));

      const parsed = JSON.parse(result.data);
      specification.value = parsed as DatasetSpecification;
      isDirty.value = true;
      currentStep.value = 'analysis';

      // Show appropriate completion message
      if (result.fallback_used) {
        statusMessage.value = 'Analysis complete (used offline fallback)!';
      } else {
        statusMessage.value = `Analysis complete (${result.mode_used} mode)!`;
      }
      await new Promise(resolve => setTimeout(resolve, 500));

      return specification.value;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      statusMessage.value = null;
      throw e;
    } finally {
      isLoading.value = false;
      setTimeout(() => {
        statusMessage.value = null;
      }, 1000);
    }
  }

  function updateSpecification(spec: DatasetSpecification) {
    specification.value = spec;
    isDirty.value = true;
  }

  function updateBatch(index: number, batch: PermutationBatch) {
    if (specification.value) {
      specification.value.permutation_batches[index] = batch;
      isDirty.value = true;
    }
  }

  function addBatch() {
    if (!specification.value) return;

    const newBatchNumber = specification.value.permutation_batches.length + 1;
    const newBatch: PermutationBatch = {
      batch_number: newBatchNumber,
      batch_name: `New Batch ${newBatchNumber}`,
      category: 'Miscellaneous',
      image_count: 40,
      prompt: `{subject1, subject2, subject3, subject4, subject5} with {modifier1, modifier2, modifier3, modifier4, modifier5, modifier6, modifier7, modifier8} --sref ${specification.value.sref_code}`,
      priority: 'medium',
      notes: '',
    };

    specification.value.permutation_batches.push(newBatch);
    isDirty.value = true;
  }

  function removeBatch(index: number) {
    if (specification.value) {
      specification.value.permutation_batches.splice(index, 1);
      // Renumber remaining batches
      specification.value.permutation_batches.forEach((batch, i) => {
        batch.batch_number = i + 1;
      });
      isDirty.value = true;
    }
  }

  function duplicateBatch(index: number) {
    if (!specification.value) return;

    const original = specification.value.permutation_batches[index];
    const newBatch: PermutationBatch = {
      ...original,
      batch_number: specification.value.permutation_batches.length + 1,
      batch_name: `${original.batch_name} (Copy)`,
    };

    specification.value.permutation_batches.push(newBatch);
    isDirty.value = true;
  }

  async function saveProject(path: string) {
    if (!specification.value) {
      throw new Error('No specification to save');
    }

    const projectData: ProjectData = {
      images: [],
      imagePaths: imagePaths.value,
      srefCode: srefCode.value,
      specification: specification.value,
      lastModified: Date.now(),
    };

    try {
      await invoke('save_project', {
        path,
        data: JSON.stringify(projectData, null, 2),
      });
      isDirty.value = false;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  async function loadProject(path: string) {
    isLoading.value = true;
    error.value = null;

    try {
      const data = await invoke<string>('load_project', { path });
      const projectData: ProjectData = JSON.parse(data);

      imagePaths.value = projectData.imagePaths;
      srefCode.value = projectData.srefCode;
      specification.value = projectData.specification;
      isDirty.value = false;

      if (specification.value) {
        currentStep.value = 'batches';
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function exportJSON(path: string) {
    if (!specification.value) {
      throw new Error('No specification to export');
    }

    try {
      const json = exportAsJSON(specification.value);
      await invoke('export_json', { path, data: json });
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  async function exportMarkdown(path: string) {
    if (!specification.value) {
      throw new Error('No specification to export');
    }

    try {
      const markdown = exportAsMarkdown(specification.value);
      await invoke('export_markdown', { path, content: markdown });
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  function goToStep(step: Step) {
    currentStep.value = step;
  }

  function reset() {
    imagePaths.value = [];
    srefCode.value = '';
    specification.value = null;
    currentStep.value = 'upload';
    isDirty.value = false;
    error.value = null;
  }

  // Settings management
  async function loadSettings() {
    try {
      const loadedSettings = await invoke<AppSettings>('get_settings');
      settings.value = loadedSettings;
      return loadedSettings;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  async function saveSettings(newSettings: AppSettings) {
    try {
      await invoke('update_settings', { settings: newSettings });
      settings.value = newSettings;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  return {
    // State
    imagePaths,
    srefCode,
    specification,
    currentStep,
    isDirty,
    isLoading,
    error,
    statusMessage,
    settings,
    lastModeUsed,
    lastFallbackUsed,

    // Computed
    hasImages,
    hasSrefCode,
    canAnalyze,
    hasSpecification,

    // Actions
    setImages,
    addImage,
    removeImage,
    setSrefCode,
    analyzeStyle,
    updateSpecification,
    updateBatch,
    addBatch,
    removeBatch,
    duplicateBatch,
    saveProject,
    loadProject,
    exportJSON,
    exportMarkdown,
    goToStep,
    reset,
    loadSettings,
    saveSettings,
  };
});
