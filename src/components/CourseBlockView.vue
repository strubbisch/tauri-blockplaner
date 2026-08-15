<script setup lang="ts">
import { Course, CourseBlock, CourseGroup } from '../stores/appStore';


interface Props {
    courseBlocks?: CourseBlock[];
    courseGroups?: CourseGroup[];
    courses?: Course[];
}

defineProps<Props>();

const getCourseName = (courseId: number, courseGroups: CourseGroup[] = [], courses: Course[] = []) => {
    const course = courses.find(c => c.id === courseId);
    const group = courseGroups.find(g => g.id === course?.course_group);
    return group ? `${group.subject}-${group.course_type}-${course?.number}` : 'Kurs nicht gefunden';
}
</script>

<template>
    <div class="rounded-lg bg-zinc-50 p-5">
        <h3 class="mt-0 text-xl font-semibold text-zinc-800">
            Kursblöcke
        </h3>
    </div>
    <div class="grid grid-cols-[2fr_1fr] gap-4">
        <div class="flex flex-col gap-3">
            <div
                v-for="block in courseBlocks"
                :key="block.id"
                class="rounded-md border border-zinc-300 bg-white p-3 shadow-sm transition hover:shadow-md"
            >
                <div class="mb-2.5 flex items-center justify-between border-b border-zinc-200 pb-2">
                    <h4 class="m-0 text-sm font-semibold text-zinc-800">{{ block.name }}</h4>
                    <span class="text-xs text-zinc-500">ID: {{ block.id }}</span>
                </div>
                <div class="mt-2 mb-1 text-xs font-semibold text-zinc-600">
                    Kurse:
                </div>
                <div class="flex flex-wrap gap-1.5">
                    <span
                    v-for="courseId in block.courses"
                    :key="courseId"
                    class="rounded bg-blue-100 px-2 py-1 text-xs text-blue-700"
                    >
                    {{ getCourseName(courseId, courseGroups, courses) }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>