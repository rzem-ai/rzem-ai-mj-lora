<template>
  <!-- Subject Distribution -->
  <Card class="grow">
    <template #content>
      <h2 class="text-xl font-bold">Subject Distribution</h2>
 
      <div>
        <div class="grid grid-cols-2 gap-6 p-2 md:grid-cols-3">
          <div v-for="(percentage, category) in training_recommendations.optimal_subject_distribution" :key="category" class="rounded bg-gray-50">
            <p class="text-sm text-gray-600">{{ category }}</p>
            <p class="text-gray-900 "></p>
            <ProgressBar :value="percentage * 100" class="h-20"><div class="text-lg font-semibold">{{ percentage * 100 }}%</div></ProgressBar>

          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import Card from 'primevue/card';
import ProgressBar from 'primevue/progressbar';

import type { TrainingRecommendations } from '../types/schema';

defineProps<{
  training_recommendations: TrainingRecommendations;
}>();
</script>
