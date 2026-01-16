<template>
  <Card>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2 p-2">
          <ImageUp class="w-5 h-5" />
          <span class="">Upload Style Reference Images</span>
        </div>
        <button
          @click="openProject"
          class="flex items-center gap-2 px-4 py-2 mr-2 font-medium text-white transition-colors bg-purple-600 rounded-lg hover:bg-purple-700"
          title="Open a saved project"
        >
          <FolderOpen class="w-4 h-4" />
          Open Project
        </button>
      </div>
    </template>
    <template #content>
      <div class="flex flex-col gap-4 p-4">
        <div v-if="statusMessage" class="flex items-center p-4 text-blue-800 bg-blue-100 rounded-lg">
          <svg class="w-5 h-5 mr-2 animate-spin" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          {{ statusMessage }}
        </div>

        <div class="flex flex-col gap-2">
          <label class="block font-semibold"> SREF Code </label>
          <InputText v-model="srefCode" fluid placeholder="Enter 10-digit SREF code" />
        </div>

        <ImageUploader :image-paths="imagePaths" @update:imagePaths="updateImages" @add="addImage" @remove="removeImage" />

        <div class="flex items-center justify-between">
          <div class="text-sm text-gray-600">
            <p v-if="imagePaths.length < 3" class="text-red-600">Upload at least 3 images</p>
            <p v-else-if="imagePaths.length > 10" class="text-yellow-600">Maximum 10 images recommended</p>
            <p v-else class="text-green-600">{{ imagePaths.length }} images uploaded</p>
            <p>canAnalyze: {{ canAnalyze }}</p>
            <p>isLoading: {{ isLoading }}</p>
            <p>isAnalyzingDisabled: {{ isAnalyzingDisabled }}</p>
          </div>

          <Button :disabled="isAnalyzingDisabled" @click="analyzeStyle">
            <WandSparkles class="w-4 h-4" />
            <div v-if="isLoading">Analyzing...</div>
            <div v-else>Analyze Style</div>
          </Button>
        </div>

        <div v-if="error" class="p-4 text-red-800 bg-red-100 rounded-lg">
          {{ error }}
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import Button from 'primevue/button';
import Card from 'primevue/card';
import InputText from 'primevue/inputtext';
import { ImageUp, WandSparkles, FolderOpen } from 'lucide-vue-next';
import { useProjectStore } from '../stores/project';
import ImageUploader from '../components/ImageUploader.vue';
import { open } from '@tauri-apps/plugin-dialog';

const store = useProjectStore();
const router = useRouter();

const imagePaths = computed(() => store.imagePaths);
const srefCode = computed({
  get: () => store.srefCode,
  set: (value) => store.setSrefCode(value),
});
const canAnalyze = computed(() => store.canAnalyze);
const isLoading = computed(() => store.isLoading);
const isAnalyzingDisabled = computed(() => !store.canAnalyze || store.isLoading);
const error = computed(() => store.error);
const statusMessage = computed(() => store.statusMessage);

const updateImages = (paths: string[]) => {
  store.setImages(paths);
};

const addImage = (path: string) => {
  store.addImage(path);
};

const removeImage = (index: number) => {
  store.removeImage(index);
};

const analyzeStyle = async () => {
  try {
    await store.analyzeStyle();
    router.push('/analysis');
  } catch (e) {
    console.error('Analysis failed:', e);
  }
};

const openProject = async () => {
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
      // Navigate to the appropriate view based on what's loaded
      if (store.specification) {
        router.push('/batches');
      }
    }
  } catch (e) {
    console.error('Failed to load project:', e);
    store.error = e instanceof Error ? e.message : String(e);
  }
};
</script>
