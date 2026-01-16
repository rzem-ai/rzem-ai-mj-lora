<template>
  <!-- Training Recommendations -->
  <Card :pt="{ body: { class: 'h-full' }, content: { class: 'h-full' } }">
    <template #content>
      <h2 class="text-xl font-bold">Training Recommendations</h2>
      <div class="text-sm font-medium">Recommended Dataset Size</div>

      <div class="flex justify-center" style="height: calc(100% - 50px);">
        <div class="content-center text-4xl font-bold">{{ training_recommendations.recommended_dataset_size }} images</div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import Card from 'primevue/card';

import type { TrainingRecommendations } from '../types/schema';

defineProps<{
  training_recommendations: TrainingRecommendations;
}>();
</script>
