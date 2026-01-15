<template>
  <div>
    <!-- File Picker Zone -->
    <div
      @click="openFilePicker"
      :class="{
        'border-blue-500 bg-blue-50 dark:bg-blue-900/20': isDragging,
        'border-gray-300 dark:border-gray-600': !isDragging,
      }"
      class="p-8 text-center transition-colors border-2 border-dashed rounded-lg cursor-pointer hover:border-blue-400 dark:hover:border-blue-500"
    >
      <div class="space-y-2">
        <svg class="w-12 h-12 mx-auto text-gray-400" stroke="currentColor" fill="none" viewBox="0 0 48 48">
          <path
            d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
        <div class="text-gray-600 dark:text-gray-400">
          <span class="font-medium text-blue-600 dark:text-blue-400 hover:text-blue-700"> Click to upload files </span>
          or drag and drop
        </div>
        <p class="text-xs text-gray-500 dark:text-gray-500">JPG, PNG or WEBP (3-10 images)</p>
      </div>
    </div>

    <!-- Image Preview Grid -->
    <div v-if="imagePaths.length > 0" class="grid grid-cols-3 gap-4 mt-6 md:grid-cols-5">
      <div v-for="(path, index) in imagePaths" :key="index" class="relative overflow-hidden bg-gray-200 rounded-lg group aspect-square dark:bg-gray-700">
        <img :src="convertFileSrc(path)" :alt="`Image ${index + 1}`" class="object-cover w-full h-full" @error="handleImageError(index)" />
        <button @click="removeImage(index)" class="absolute p-1 text-white transition-opacity bg-red-600 rounded-full opacity-0 top-2 right-2 group-hover:opacity-100">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
        <div class="absolute bottom-0 left-0 right-0 p-1 text-xs text-white truncate bg-black bg-opacity-50">
          {{ getFileName(path) }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import type { UnlistenFn } from '@tauri-apps/api/event';

defineProps<{
  imagePaths: string[];
}>();

const emit = defineEmits<{
  (e: 'update:imagePaths', paths: string[]): void;
  (e: 'add', path: string): void;
  (e: 'remove', index: number): void;
}>();

const isDragging = ref(false);
let unlistenDrop: UnlistenFn | null = null;
let unlistenDragOver: UnlistenFn | null = null;
let unlistenDragLeave: UnlistenFn | null = null;

onMounted(async () => {
  try {
    const appWindow = getCurrentWindow();
    console.log('Setting up drag-drop event listener...');

    // Listen for file drop events
    unlistenDrop = await appWindow.onDragDropEvent((event) => {
      console.log('Drag-drop event:', event.payload.type, event.payload);

      if (event.payload.type === 'drop') {
        isDragging.value = false;
        const paths = event.payload.paths as string[];
        console.log('Files dropped:', paths);

        // Filter for image files only
        const imagePaths = paths.filter((path) => {
          const ext = path.toLowerCase().split('.').pop();
          return ['jpg', 'jpeg', 'png', 'webp'].includes(ext || '');
        });

        console.log('Image files filtered:', imagePaths);

        // Add each image
        for (const path of imagePaths) {
          emit('add', path);
        }
      } else if (event.payload.type === 'enter') {
        console.log('Drag entered window');
        isDragging.value = true;
      } else if (event.payload.type === 'leave') {
        console.log('Drag left window');
        isDragging.value = false;
      }
    });

    console.log('Drag-drop listener set up successfully');
  } catch (error) {
    console.error('Error setting up drag-drop listener:', error);
  }
});

onUnmounted(() => {
  // Clean up event listeners
  if (unlistenDrop) unlistenDrop();
  if (unlistenDragOver) unlistenDragOver();
  if (unlistenDragLeave) unlistenDragLeave();
});

const openFilePicker = async () => {
  try {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: 'Images',
          extensions: ['jpg', 'jpeg', 'png', 'webp'],
        },
      ],
    });

    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      for (const path of paths) {
        emit('add', path);
      }
    }
  } catch (error) {
    console.error('Error opening file picker:', error);
  }
};

const removeImage = (index: number) => {
  emit('remove', index);
};

const getFileName = (path: string) => {
  return path.split('/').pop() || path.split('\\').pop() || path;
};

const handleImageError = (index: number) => {
  console.error(`Failed to load image at index ${index}`);
};
</script>
