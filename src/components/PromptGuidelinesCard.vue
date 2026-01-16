<template>
  <Card>
    <template #content>
      <div class="flex flex-col gap-4">
        <div class="text-xl font-bold">Prompt Guidelines</div>

        <div class="space-y-4">
          <div v-if="prompt_guidelines.keep_simple" class="flex gap-2 place-items-center"><CircleCheckBig class="w-5 h-5 text-green-600" /> Keep prompts simple - let SREF handle styling</div>

          <div class="space-y-2">
            <div class="text-lg font-medium">Avoid These Keywords</div>

            <div class="flex flex-wrap gap-2">
              <Tag v-for="keyword in prompt_guidelines.avoid_style_keywords" :key="keyword" severity="danger" class="flex items-center gap-1 px-2 bg-gray-100 border rounded-lg shadow">
                {{ keyword }}
              </Tag>
            </div>
          </div>

          <div class="space-y-2">
            <div class="text-lg font-medium">Recommended Additions</div>

            <div class="flex flex-wrap gap-2">
              <Tag v-for="addition in prompt_guidelines.recommended_additions" :key="addition" severity="success" class="flex items-center gap-1 px-2 bg-gray-100 border rounded-lg shadow">
                {{ addition }}
              </Tag>
            </div>
          </div>
        </div>
      </div>
    </template>
  </Card>
</template>

<script setup lang="ts">
import Card from 'primevue/card';
import Tag from 'primevue/tag';
import { CircleCheckBig, Eye, CircleCheck, CircleX, CircleDot } from 'lucide-vue-next';
import type { PromptGuidelines } from '../types/schema';

defineProps<{
  prompt_guidelines: PromptGuidelines;
}>();
</script>
