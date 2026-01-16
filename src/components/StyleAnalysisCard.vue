<template>
  <Card>
    <template #content>
      <div class="flex flex-col gap-4">
        <div class="text-xl font-bold">Style Analysis</div>
        <div>
          <div class="text-lg font-medium">Primary Style</div>
          <UseClipboard v-slot="{ copy, copied }" :source="analysis.primary_style">
            <InputText v-model="analysis.primary_style" @click="copy()" disabled fluid />
          </UseClipboard>
        </div>

        <div>
          <div class="text-lg font-medium">Era Influence</div>
          <UseClipboard v-slot="{ copy, copied }" :source="analysis.era_influence">
            <InputText v-model="analysis.era_influence" @click="copy()" disabled fluid />
          </UseClipboard>
        </div>

        <div>
          <div class="text-lg font-medium">Color Palette</div>
          <div class="flex flex-wrap gap-2">
            <Tag v-for="color in analysis.color_palette" :key="color" class="flex items-center gap-1 px-2 bg-gray-100 border rounded-lg shadow">
              <Palette class="w-4 h-4" />
              {{ color }}
            </Tag>
          </div>
        </div>

        <div>
          <div class="text-lg font-medium">Key Characteristics</div>
          <div class="grid grid-cols-2 gap-1">
            <div v-for="char in analysis.key_characteristics" :key="char" class="flex gap-1 text-sm place-items-center">
              <Dot class="w-3 h-3" />
              {{ char }}
            </div>
          </div>
        </div>

        <div class="flex w-full">
          <div class="grow">
            <div class="text-lg font-medium">Best Subjects</div>
            <div class="grid grid-cols-1 gap-1">
              <div v-for="subject in analysis.best_subjects" :key="subject" class="flex gap-1 text-sm place-items-center">
                <Dot class="w-3 h-3" />
                {{ subject }}
              </div>
            </div>
          </div>

          <div class="grow">
            <div class="text-lg font-medium">Avoid Subjects</div>
            <div class="grid grid-cols-1 gap-1">
              <div v-for="subject in analysis.avoid_subjects" :key="subject" class="flex gap-1 text-sm place-items-center">
                <Dot class="w-3 h-3" />
                {{ subject }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import Tag from 'primevue/tag';
import Card from 'primevue/card';
import InputText from 'primevue/inputtext';
import { UseClipboard } from '@vueuse/components';
import { ImageUp, Dot, CircleCheck, CircleX, Palette } from 'lucide-vue-next';
import type { StyleAnalysis } from '../types/schema';

defineProps<{
  analysis: StyleAnalysis;
}>();
</script>
