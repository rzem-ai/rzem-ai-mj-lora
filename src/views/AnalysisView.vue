<template>
  <div v-if="specification" class="flex flex-col gap-4">
    <!-- Style Analysis Card -->
    <StyleAnalysisCard :analysis="specification.style_analysis" />

    <!-- Training Recommendations -->
    <div class="flex gap-4">
      <TrainingRecommendationsCard :training_recommendations="specification.training_recommendations" />
      <SubjectDistributionCard :training_recommendations="specification.training_recommendations" />
    </div>

    <!-- Prompt Guidelines -->
    <PromptGuidelinesCard :prompt_guidelines="specification.prompt_guidelines"/>

    

    <!-- Navigation -->
    <div class="flex justify-between">
      <button @click="goBack" class="px-6 py-3 font-medium text-gray-900 transition-colors bg-gray-300 rounded-lg dark:bg-gray-700 dark:text-white hover:bg-gray-400 dark:hover:bg-gray-600">
        Back
      </button>
      <button @click="goToBatches" class="px-6 py-3 font-medium text-white transition-colors bg-blue-600 rounded-lg hover:bg-blue-700">Edit Batches</button>
    </div>
  </div>
  <Card v-else class="space-y-6">
    <template #content>
      <div class="py-12 text-center text-gray-600 dark:text-gray-400">No analysis available. Please upload images and analyze first.</div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import Button from 'primevue/button';
import Card from 'primevue/card';
import { ImageUp } from 'lucide-vue-next';
import { useProjectStore } from '../stores/project';
import StyleAnalysisCard from '../components/StyleAnalysisCard.vue';
import SubjectDistributionCard from '../components/SubjectDistributionCard.vue';
import TrainingRecommendationsCard from '../components/TrainingRecommendationsCard.vue';
import PromptGuidelinesCard from '../components/PromptGuidelinesCard.vue';

const store = useProjectStore();
const router = useRouter();

const specification = computed(() => store.specification);

const goBack = () => {
  router.push('/');
};

const goToBatches = () => {
  store.goToStep('batches');
  router.push('/batches');
};
</script>
