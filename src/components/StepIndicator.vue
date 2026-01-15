<template>
  <button
    class="flex items-center justify-center gap-1 px-4 py-2 font-medium transition-colors rounded-lg shadow"
    :class="{
      'bg-blue-400 text-white': active,
      'bg-green-600 text-white': completed && !active,
      'bg-gray-400 text-gray-50': !active && !completed,
      'cursor-wait hover:bg-gray-500': !active,
    }"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
defineProps<{
  active: boolean;
  completed: boolean;
}>();
</script>
