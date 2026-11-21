<script setup lang="ts">
import { Block, AppData, Course, CourseUnit, BlockAssignment, CourseGroup } from '../stores/appStore';


interface Props {
    blocks?: Block[];
    isLoading?: boolean;
    appData?: AppData;
}

interface CourseInBlock {
    course: Course;
    courseUnit: CourseUnit;
}

const props = defineProps<Props>();

const getCoursesInBlock = (blockId: number): CourseInBlock[] => {
    if (!props.appData) return [];
    
    const blockAssignments = props.appData.block_assignments.filter((ba: BlockAssignment) => ba.block_id === blockId);
    const courses: CourseInBlock[] = [];
    
    for (const assignment of blockAssignments) {
        const courseUnit = props.appData.course_units.find((cu: CourseUnit) => cu.id === assignment.course_unit_id);
        if (courseUnit) {
            const course = props.appData.courses.find((c: Course) => c.id === courseUnit.course_id);
            if (course) {
                courses.push({ course, courseUnit });
            }
        }
    }
    
    return courses;
};

const getCourseGroupName = (courseGroupId: number): string => {
    if (!props.appData) return '';
    const courseGroup = props.appData.course_groups.find((cg: CourseGroup) => cg.id === courseGroupId);
    return courseGroup ? courseGroup.subject + "-" + courseGroup.course_type : '';
};

const getStudentCountInBlock = (blockId: number): number => {
    const coursesInBlock = getCoursesInBlock(blockId);
    const studentIds = new Set<number>();
    
    for (const { course } of coursesInBlock) {
        for (const studentId of course.students) {
            studentIds.add(studentId);
        }
    }
    
    return studentIds.size;
};

const getTotalStudents = (): number => {
    if (!props.appData) return 0;
    return props.appData.students.length;
};

const getBlockStudentPercentage = (blockId: number): number => {
    const totalStudents = getTotalStudents();
    if (totalStudents === 0) return 0;
    
    const blockStudents = getStudentCountInBlock(blockId);
    return Math.round((blockStudents / totalStudents) * 100);
};

</script>

<template>
    <div class="rounded-lg bg-zinc-50 p-5">
        <h3 class="mt-0 text-xl font-semibold text-zinc-800">Blöcke</h3>

        <div v-if="isLoading" class="p-5 text-center text-sm text-zinc-600">Loading...</div>
    
        <div v-else-if="!blocks || blocks.length === 0" class="p-5 text-center text-sm text-zinc-600">
            <p>Bisher keine Blockung berechnet.</p>
        </div>

        <div v-else class="flex flex-col gap-3">
            <div
                v-for="block in blocks"
                :key="block.id"
                class="rounded-md border border-zinc-300 bg-white p-3 shadow-sm transition hover:shadow-md"
            >
                <div class="mb-2.5 flex items-center justify-between border-b border-zinc-200 pb-2">
                    <h4 class="m-0 text-sm font-semibold text-zinc-800">
                        Block {{ block.number }} ({{ block.length === 'Double' ? 2 : 1 }}h) 
                        - {{ getStudentCountInBlock(block.id) }} Schüler ({{ getBlockStudentPercentage(block.id) }}%)
                    </h4>
                </div>
                
                <div class="space-y-2">
                    <div
                        v-for="{ course, courseUnit } in getCoursesInBlock(block.id)"
                        :key="course.id"
                        class="rounded border border-zinc-200 bg-zinc-50 p-2 text-sm text-zinc-700"
                    >
                        <div class="flex items-center justify-between">
                            <span class="font-medium">{{ getCourseGroupName(course.course_group) }}-{{ course.number }}</span>
                            <span class="text-zinc-600">{{ courseUnit.length === 'Double' ? 2 : 1 }}h · {{ course.students.length }} Schüler</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>