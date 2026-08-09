<script setup lang="ts">
import { CourseGroupWithCount } from '../stores/appStore';

interface Props {
  courseGroups: CourseGroupWithCount[];
  isLoading?: boolean;
}

defineProps<Props>();
</script>

<template>
  <div class="rounded-lg bg-zinc-50 p-5">
    <h3 class="mt-0 text-center text-xl font-semibold text-zinc-800">Kursgruppen</h3>
    
    <div v-if="isLoading" class="p-5 text-center text-sm text-zinc-600">Loading...</div>
    
    <div v-else-if="courseGroups.length === 0" class="p-5 text-center text-sm text-zinc-600">
      <p>Bisher keine Kursgruppen importiert.</p>
    </div>
    
    <div v-else class="grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-3">
      <div
        v-for="group in courseGroups"
        :key="group.id"
        class="rounded-md border border-zinc-300 bg-white p-3 shadow-sm transition hover:shadow-md"
      >
        <div class="mb-2 flex items-center justify-between">
          <h4 class="m-0 text-sm font-semibold text-blue-500">{{ group.subject }}</h4>
          <span
            class="rounded-xl bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700"
          >
            {{ group.course_type }}
          </span>
        </div>
        <div class="text-xs text-zinc-500">Anzahl Schüler: {{ group.students_count }}</div>
      </div>
    </div>
  </div>
</template>
