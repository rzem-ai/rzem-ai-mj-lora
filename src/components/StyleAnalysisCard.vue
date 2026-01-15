<template>
  <Card>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2 p-2">
          <Eye class="w-5 h-5" />
          <span class="">Style Analysis</span>
        </div>
      </div>
    </template>
    <template #content>
      <div class="flex flex-col gap-4 p-4">
        <div>
          <h2 class="">Primary Style</h2>
          <UseClipboard v-slot="{ copy, copied }" :source="analysis.primary_style">
            <div class="copy-text" @click="copy()">{{ analysis.primary_style }}</div>
          </UseClipboard>
        </div>

        <div>
          <h2 class="">Era Influence</h2>
          <UseClipboard v-slot="{ copy, copied }" :source="analysis.era_influence">
            <div class="copy-text" @click="copy()">{{ analysis.era_influence }}</div>
          </UseClipboard>
        </div>

        <div>
          <h2 class="">Color Palette</h2>
          <div class="flex flex-wrap gap-2">
            <span v-for="color in analysis.color_palette" :key="color" class="px-3 py-1 text-sm text-purple-800 bg-purple-100 rounded-full dark:bg-purple-900/20 dark:text-purple-400">
              {{ color }}
            </span>
          </div>
        </div>

        <div>
          <h3 class="">Key Characteristics</h3>
          <ul class="space-y-1 text-gray-900 list-disc list-inside dark:text-white">
            <li v-for="char in analysis.key_characteristics" :key="char">{{ char }}</li>
          </ul>
        </div>

        <div class="grid gap-4 md:grid-cols-2">
          <div>
            <h3 class="">Best Subjects</h3>
            <ul class="space-y-1 text-sm text-gray-900 list-disc list-inside dark:text-white">
              <li v-for="subject in analysis.best_subjects" :key="subject">{{ subject }}</li>
            </ul>
          </div>

          <div>
            <h3 class="mb-2 text-sm font-medium text-red-700 dark:text-red-400">Avoid Subjects</h3>
            <ul class="space-y-1 text-sm text-gray-900 list-disc list-inside dark:text-white">
              <li v-for="subject in analysis.avoid_subjects" :key="subject">{{ subject }}</li>
            </ul>
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import Button from 'primevue/button';
import Card from 'primevue/card';
import { UseClipboard } from '@vueuse/components';
import { ImageUp, Eye } from 'lucide-vue-next';
import type { StyleAnalysis } from '../types/schema';

defineProps<{
  analysis: StyleAnalysis;
}>();
</script>
