<template>
  <div class="flex flex-col min-h-screen bg-gray-200 resize-container-6">
    <!-- Header -->
    <header class="shadow bg-gray-50">
      <div class="flex items-center justify-between px-4 py-4">
        <div>
          <h1 class="text-2xl font-bold ">LoRA Training Dataset Generator</h1>
          <p class="mt-1 text-sm text-gray-600">Generate optimized Midjourney permutation batches for LoRA training</p>
        </div>
        <button
          @click="goToSettings"
          class="flex items-center gap-2 px-4 py-2 text-gray-700 transition-colors rounded-lg hover:bg-gray-200"
          title="Settings"
        >
          <Settings class="w-5 h-5" />
          <span class="hidden sm:inline">Settings</span>
        </button>
      </div>
    </header>

    <!-- Progress Stepper -->
    <div class="w-full px-4 py-6">
      <div class="flex items-center justify-center space-x-4">
        <StepIndicator :active="currentStep === 'upload'" :completed="isStepCompleted('upload')" @click="goToStep('upload')"> <ImageUp class="w-4 h-4"/> Upload </StepIndicator>
        <div class="h-0.5 w-16 bg-gray-400" />
        <StepIndicator :active="currentStep === 'analysis'" :completed="isStepCompleted('analysis')" @click="goToStep('analysis')"> <Eye class="w-4 h-4"/> Analysis </StepIndicator>
        <div class="h-0.5 w-16 bg-gray-400" />
        <StepIndicator :active="currentStep === 'batches'" :completed="isStepCompleted('batches')" @click="goToStep('batches')"> <SquarePen class="w-4 h-4"/> Edit Batches </StepIndicator>
        <div class="h-0.5 w-16 bg-gray-400" />
        <StepIndicator :active="currentStep === 'export'" :completed="isStepCompleted('export')" @click="goToStep('export')"> <FileDown class="w-4 h-4"/> Export </StepIndicator>
      </div>
    </div>

    <!-- Main Content -->
    <main class="w-full px-4 pb-8">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';
import { useProjectStore } from './stores/project';
import StepIndicator from './components/StepIndicator.vue';
import { Eye, FileDown, ImageUp, WandSparkles, SquarePen, Settings } from 'lucide-vue-next';

const store = useProjectStore();
const router = useRouter();

const currentStep = computed(() => store.currentStep);

const isStepCompleted = (step: string) => {
  const steps = ['upload', 'analysis', 'batches', 'export'];
  const currentIndex = steps.indexOf(store.currentStep);
  const stepIndex = steps.indexOf(step);
  return stepIndex < currentIndex;
};

const goToStep = (step: string) => {
  // Only allow navigation if we have the necessary data
  if (step === 'upload') {
    router.push('/');
  } else if (step === 'analysis' && store.hasSpecification) {
    router.push('/analysis');
  } else if (step === 'batches' && store.hasSpecification) {
    router.push('/batches');
  } else if (step === 'export' && store.hasSpecification) {
    router.push('/export');
  }
};

const goToSettings = () => {
  router.push('/settings');
};
</script>
