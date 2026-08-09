<script setup lang="ts">
import type { Student, CourseGroup, Course } from "../stores/appStore";

interface Props {
  students?: Student[];
  courseGroups?: CourseGroup[];
  courses?: Course[];
  isLoading?: boolean;
}

defineProps<Props>();

const getCourseGroupName = (courseGroupId: number, courseGroups: CourseGroup[] = []) => {
  const group = courseGroups.find(g => g.id === courseGroupId);
  return group ? `${group.subject}-${group.course_type}` : `Course ${courseGroupId}`;
};

const getCourseName = (courseId: number, courses: Course[] = [], courseGroups: CourseGroup[] = []) => {
  const course = courses.find(c => c.id === courseId);
  const group = courseGroups.find(g => g.id === course?.course_group);
  return group ? `${group.subject}-${group.course_type}-${course?.number}` : 'Kurs nicht gefunden';
}
</script>

<template>
  <div class="rounded-lg bg-zinc-50 p-5">
    <h3 class="mt-0 text-xl font-semibold text-zinc-800">Schüler</h3>
    
    <div v-if="isLoading" class="p-5 text-center text-sm text-zinc-600">Loading...</div>
    
    <div v-else-if="!students || students.length === 0" class="p-5 text-center text-sm text-zinc-600">
      <p>Bisher keine Schüler importiert.</p>
    </div>
    
    <div v-else class="flex flex-col gap-3">
      <div
        v-for="student in students"
        :key="student.id"
        class="rounded-md border border-zinc-300 bg-white p-3 shadow-sm transition hover:shadow-md"
      >
        <div class="mb-2.5 flex items-center justify-between border-b border-zinc-200 pb-2">
          <h4 class="m-0 text-sm font-semibold text-zinc-800">{{ student.name }}</h4>
          <span class="text-xs text-zinc-500">ID: {{ student.id }}</span>
        </div>
        
        <div v-if="student.choices && student.choices.length > 0" class="mt-2">
          <div class="mb-1 text-xs font-semibold uppercase text-zinc-600">gewählte Kurse:</div>
          <div class="flex flex-wrap gap-1.5">
            <span
              v-for="choiceId in student.choices"
              :key="choiceId"
              class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-700"
            >
              {{ getCourseGroupName(choiceId, courseGroups) }}
            </span>
          </div>
        </div>
        
        <div v-if="student.courses && student.courses.length > 0" class="mt-2">
          <div class="mb-1 text-xs font-semibold uppercase text-zinc-600">zugeordnete Kurse:</div>
          <div class="flex flex-wrap gap-1.5">
            <span
              v-for="courseId in student.courses"
              :key="courseId"
              class="rounded bg-green-100 px-2 py-1 text-xs text-green-700"
            >
              {{ getCourseName(courseId, courses, courseGroups) }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
