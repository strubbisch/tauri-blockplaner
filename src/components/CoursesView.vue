<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { Course, CourseGroup } from "../stores/appStore";
import { useAppStore } from "../stores/appStore";

interface CourseWithGroup extends Course {
  groupName: string;
}

interface Props {
  courses: Course[];
  courseGroups: CourseGroup[];
  isLoading?: boolean;
}

const props = defineProps<Props>();
const appStore = useAppStore();
const selectedCourseId = ref<number | null>(null);
const formTeacher = ref("");
const formRoom = ref("");
const formHoursPerWeek = ref(0);

const getCourseGroupName = (courseGroupId: number, courseGroups: CourseGroup[]) => {
  const group = courseGroups.find((item) => item.id === courseGroupId);
  return group ? `${group.subject}-${group.course_type}` : `Kursgruppe ${courseGroupId}`;
};

const coursesWithGroup = computed<CourseWithGroup[]>(() =>
  props.courses.map((course) => ({
    ...course,
    groupName: getCourseGroupName(course.course_group, props.courseGroups),
  })),
);

const selectedCourse = computed(() =>
  props.courses.find((course) => course.id === selectedCourseId.value) ?? null,
);

const normalizeCourseFields = (teacher: unknown, room: unknown, hoursPerWeek: unknown) => {
  const normalizedTeacher = typeof teacher === "string" ? teacher.trim() : "";
  const normalizedRoom = typeof room === "string" ? room.trim() : "";
  const normalizedHours =
    typeof hoursPerWeek === "number" ? hoursPerWeek : Number(hoursPerWeek);

  return {
    teacher: normalizedTeacher,
    room: normalizedRoom,
    hoursPerWeek: Number.isFinite(normalizedHours) ? Math.max(0, Math.trunc(normalizedHours)) : 0,
  };
};

const selectedCourseNormalized = computed(() => {
  if (!selectedCourse.value) {
    return null;
  }

  return normalizeCourseFields(
    selectedCourse.value.teacher,
    selectedCourse.value.room,
    selectedCourse.value.hours_per_week,
  );
});

const formNormalized = computed(() =>
  normalizeCourseFields(formTeacher.value, formRoom.value, formHoursPerWeek.value),
);

const isDirty = computed(() => {
  if (!selectedCourseNormalized.value) {
    return false;
  }

  return (
    selectedCourseNormalized.value.teacher !== formNormalized.value.teacher ||
    selectedCourseNormalized.value.room !== formNormalized.value.room ||
    selectedCourseNormalized.value.hoursPerWeek !== formNormalized.value.hoursPerWeek
  );
});

const isLoadingState = computed(() => props.isLoading ?? false);

watch(
  selectedCourse,
  (course) => {
    if (!course) return;
    const normalizedCourse = normalizeCourseFields(
      course.teacher,
      course.room,
      course.hours_per_week,
    );
    formTeacher.value = normalizedCourse.teacher;
    formRoom.value = normalizedCourse.room;
    formHoursPerWeek.value = normalizedCourse.hoursPerWeek;
  },
  { immediate: true },
);

watch(
  () => props.courses,
  (courses) => {
    if (
      selectedCourseId.value !== null &&
      !courses.some((course) => course.id === selectedCourseId.value)
    ) {
      selectedCourseId.value = null;
    }
  },
);

const selectCourse = (course: Course) => {
  selectedCourseId.value = course.id;
  if (!course) {
    return;
  }
  const normalizedCourse = normalizeCourseFields(
    course.teacher,
    course.room,
    course.hours_per_week,
  );
  formTeacher.value = normalizedCourse.teacher;
  formRoom.value = normalizedCourse.room;
  formHoursPerWeek.value = normalizedCourse.hoursPerWeek;
};

const updateSelectedCourse = async () => {
  if (!selectedCourse.value || !isDirty.value) {
    return;
  }

  await appStore.updateCourse(
    selectedCourse.value.id,
    formNormalized.value.teacher,
    formNormalized.value.room,
    formNormalized.value.hoursPerWeek,
  );

  selectedCourseId.value = null;
};

const createParallelCourse = async () => {
  if (!selectedCourse.value) {
    return;
  }

  await appStore.createParallelCourse(
    selectedCourse.value.course_group,
    selectedCourse.value.hours_per_week,
  );

  selectedCourseId.value = null;
}

const deleteCourse = async () => {
  if (!selectedCourse.value) {
    return;
  }

  await appStore.deleteCourse(
    selectedCourse.value.course_group,
  );
  selectedCourseId.value = null;
}

const createCourses = async () => {
  await appStore.createCourses();
}

</script>

<template>
  <div class="flex flex-col gap-4 rounded-lg bg-zinc-50 p-5">
    <h3 class="m-0 text-center text-xl font-semibold text-zinc-800">Kurse</h3>

    <div v-if="isLoading" class="p-5 text-center text-sm text-zinc-600">Loading...</div>

    <div v-else-if="courses.length === 0" class="p-5 text-center text-sm text-zinc-600">
      <p>Bisher keine Kurse importiert.</p>
      <button
        @click="createCourses"
        class="btn-primary"
      >
        Kurse erstellen
      </button>
    </div>

    <div v-else class="grid grid-cols-1 items-start gap-4 xl:grid-cols-[2fr_1fr]">
      <table class="border-collapse w-full text-left">
        <thead class="border-b border-gray-300">
          <tr>
            <th>Kurs</th>
            <th>Wochenstunden</th>
            <th>Lehrer</th>
            <th>Raum</th>
            <th>Anzahl Schüler</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="course in coursesWithGroup" class="even:bg-gray-200 cursor-pointer hover:bg-blue-100"
            :key="course.id"
            @click="selectCourse(course)"
          >
            <td>{{ course.groupName }}-{{ course.number }}</td>
            <td>{{ course.hours_per_week }}</td>
            <td>{{ course.teacher }}</td>
            <td>{{ course.room }}</td>
            <td>{{ course.students.length }}</td>
          </tr>
        </tbody>
      </table>

      <div class="sticky top-0">
        <form
          v-if="selectedCourse"
          class="rounded-lg border border-zinc-300 bg-white p-3.5"
          @submit.prevent="updateSelectedCourse"
        >
          <h4 class="mb-3 mt-0 text-base font-semibold">Kurs bearbeiten</h4>
          <div class="mb-2 text-[13px]"><strong>ID:</strong> {{ selectedCourse.id }}</div>
          <div class="mb-2 text-[13px]">
            <strong>Kurs:</strong>
            {{ getCourseGroupName(selectedCourse.course_group, courseGroups) }}-{{ selectedCourse.number }}
          </div>
          <div class="mb-2 text-[13px]"><strong>Schüler:</strong> {{ selectedCourse.students.length }}</div>

          <label class="mb-2.5 grid gap-1 text-[13px]">
            <span>Lehrkraft</span>
            <input
              v-model="formTeacher"
              class="rounded-lg border border-zinc-300 bg-white px-3 py-2 text-sm outline-none shadow focus:border-blue-600"
            />
          </label>

          <label class="mb-2.5 grid gap-1 text-[13px]">
            <span>Raum</span>
            <input
              v-model="formRoom"
              class="rounded-lg border border-zinc-300 bg-white px-3 py-2 text-sm outline-none shadow focus:border-blue-600"
            />
          </label>

          <label class="mb-2.5 grid gap-1 text-[13px]">
            <span>Stunden/Woche</span>
            <input
              v-model.number="formHoursPerWeek"
              type="number"
              min="0"
              step="1"
              required
              class="rounded-lg border border-zinc-300 bg-white px-3 py-2 text-sm outline-none shadow focus:border-blue-600"
            />
          </label>

          <div
            v-if="appStore.error.value"
            class="mt-2.5 rounded border border-red-300 bg-red-100 p-2.5 text-[13px] text-red-900"
          >
            {{ appStore.error.value }}
          </div>

          <div class="mt-3 flex gap-2">
            <button
              type="submit"
              :disabled="isLoadingState || !isDirty"
              class="btn btn-primary"
            >
              {{ isLoadingState ? "Aktualisiere..." : "Aktualisieren" }}
            </button>
            <button
              @click="deleteCourse"
              class="btn btn-primary"
            >
              Kurs löschen
            </button>
            <button
              @click="createParallelCourse"
              class="btn btn-primary"
            >
              Parallelkurs anlegen
            </button>
          </div>
        </form>

        <div v-else class="rounded-lg border border-zinc-300 bg-white p-3.5 text-sm text-zinc-600">
          Kurs anklicken, um die Daten zu bearbeiten.
        </div>
      </div>
    </div>
  </div>
</template>
