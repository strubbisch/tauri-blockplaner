<script setup lang="ts">
import { ref } from 'vue';
import { Course, CourseBlock, CourseGroup, useAppStore } from '../stores/appStore';


interface Props {
    courseBlocks?: CourseBlock[];
    courseGroups?: CourseGroup[];
    courses?: Course[];
}

defineProps<Props>();

const selectedCourseBlock = ref<CourseBlock | null>(null);
const appStore = useAppStore();

const selectCourseBlock = (courseBlock: CourseBlock) => {
    selectedCourseBlock.value = courseBlock;
}

const getCourseName = (courseId: number, courseGroups: CourseGroup[] = [], courses: Course[] = []) => {
    const course = courses.find(c => c.id === courseId);
    const group = courseGroups.find(g => g.id === course?.course_group);
    return group ? `${group.subject}-${group.course_type}-${course?.number}` : 'Kurs nicht gefunden';
}

const newCourseBlock = async () => {
    await appStore.newCourseBlock();
}

const updateCourseBlock = async () => {
    if (selectedCourseBlock.value === null) {
        return;
    }
    await appStore.updateCourseBlock(
        selectedCourseBlock.value.id,
        selectedCourseBlock.value.name,
        selectedCourseBlock.value.courses,
    );
    selectedCourseBlock.value = null;
}
</script>

<template>
    <div class="flex flex-col gap-4 rounded-lg bg-zinc-50 p-5">
        <div class="rounded-lg bg-zinc-50 p-5">
            <h3 class="mt-0 text-xl font-semibold text-zinc-800">
                Kursblöcke
            </h3>
        </div>
        <div>
            <button
                class="btn btn-primary"
                @click="newCourseBlock()"
            >
                Neuer Kursblock
            </button>
        </div>
        <div class="grid grid-cols-[2fr_1fr] gap-4">
            <div class="flex flex-col gap-3">
                <div
                    v-for="block in courseBlocks"
                    :key="block.id"
                    class="rounded-md border border-zinc-300 bg-white p-3 shadow-sm transition hover:shadow-md"
                    @click="selectCourseBlock(block)"
                >
                    <div
                        class="mb-2.5 flex items-center justify-between border-b border-zinc-200 pb-2"
                    >
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
            <div class="top-0 sticky">
                <form
                    v-if="selectedCourseBlock"
                    class="rounded-lg border border-zinc-300 bg-white p-3.5"
                >
                    <h4 class="mb-3 mt-0 text-base font-semibold">Kursblock bearbeiten</h4>
                    <label class="mb-2.5 grid gap-1 text-[13px]">
                        <span>Blockname</span>
                        <input
                            class="rounded-lg border border-zinc-300 bg-white px-3 py-2 text-sm outline-none shadow focus:border-blue-600"
                            v-model="selectedCourseBlock.name"
                        />
                    </label>
                    <label class="mb-2.5 grid gap-1 text-[13px]">
                        <span>Kurse, die zusammenliegen sollen</span>
                        <select
                            class="rounded-lg border border-zinc-300 bg-white px-3 py-2 text-sm outline-none shadow focus:border-blue-600"
                            v-model="selectedCourseBlock.courses"
                            multiple
                            size="10"
                        >
                            <option v-for="course in courses" :value="course.id">
                                {{ getCourseName(course.id, courseGroups, courses) }}
                            </option>
                        </select>
                    </label>
                    <div class="mt-3 flex gap-2">
                        <button
                        type="submit"
                        class="btn btn-primary"
                        :disabled="!selectedCourseBlock"
                        @click="updateCourseBlock()"
                        >
                            Aktualisieren
                        </button>
                        <button
                        class="btn btn-primary"
                        >
                            Kursblock löschen
                        </button>
                    </div>
                </form>
                <div v-else class="rounded-lg border border-zinc-300 bg-white p-3.5 text-sm text-zinc-600">
                    Kursblock anklicken, um die Daten zu bearbeiten.
                </div>
            </div>
        </div>
    </div>
</template>