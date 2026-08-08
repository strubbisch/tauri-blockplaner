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

const selectCourse = (courseId: number) => {
  const course = props.courses.find((item) => item.id === courseId);
  selectedCourseId.value = courseId;
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
</script>

<template>
  <div class="courses-view">
    <h3>Kurse</h3>

    <div v-if="isLoading" class="loading">Loading...</div>

    <div v-else-if="courses.length === 0" class="empty">
      <p>Bisher keine Kurse importiert.</p>
    </div>

    <div v-else class="courses-content">
      <div class="courses-grid">
        <button
          v-for="course in coursesWithGroup"
          :key="course.id"
          class="course-card"
          :class="{ selected: selectedCourseId === course.id }"
          type="button"
          @click="selectCourse(course.id)"
        >
          <div class="course-header">
            <h4>{{ course.groupName }}-{{ course.number }}</h4>
            <span class="students-count">{{ course.students.length }} Schüler</span>
          </div>
          <div class="course-meta">
            <div><strong>Lehrkraft:</strong> {{ course.teacher }}</div>
            <div><strong>Raum:</strong> {{ course.room }}</div>
            <div><strong>Stunden/Woche:</strong> {{ course.hours_per_week }}</div>
          </div>
        </button>
      </div>

      <form v-if="selectedCourse" class="course-form" @submit.prevent="updateSelectedCourse">
        <h4>Kurs bearbeiten</h4>
        <div class="meta-row"><strong>ID:</strong> {{ selectedCourse.id }}</div>
        <div class="meta-row"><strong>Kurs:</strong> {{ getCourseGroupName(selectedCourse.course_group, courseGroups) }}-{{ selectedCourse.number }}</div>
        <div class="meta-row"><strong>Schüler:</strong> {{ selectedCourse.students.length }}</div>

        <label class="field">
          <span>Lehrkraft</span>
          <input v-model="formTeacher" />
        </label>

        <label class="field">
          <span>Raum</span>
          <input v-model="formRoom" />
        </label>

        <label class="field">
          <span>Stunden/Woche</span>
          <input v-model.number="formHoursPerWeek" type="number" min="0" step="1" required />
        </label>

        <div v-if="appStore.error.value" class="alert alert-error">
          {{ appStore.error.value }}
        </div>

        <div class="form-actions">
          <button type="submit" :disabled="isLoadingState || !isDirty">
            {{ isLoadingState ? "Aktualisiere..." : "Aktualisieren" }}
          </button>
          <button>Kurs löschen</button>
          <button @click="createParallelCourse">Parallelkurs anlegen</button>
        </div>
      </form>

      <div v-else class="course-form-empty">
        Kurs anklicken, um die Daten zu bearbeiten.
      </div>
    </div>
  </div>
</template>

<style scoped>
.courses-view {
  padding: 20px;
  border-radius: 8px;
  background-color: #f9f9f9;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.courses-view h3 {
  margin: 0;
  color: #333;
  text-align: center;
}

.loading,
.empty {
  padding: 20px;
  text-align: center;
  color: #666;
  font-size: 14px;
}

.courses-content {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 16px;
  align-items: start;
}

.courses-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 12px;
}

.course-card {
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
  text-align: left;
}

.course-card:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.course-card.selected {
  border-color: #007bff;
}

.course-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.course-header h4 {
  margin: 0;
  color: #007bff;
  font-size: 15px;
}

.students-count {
  background-color: #e7f3ff;
  color: #0056b3;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
}

.course-meta {
  display: grid;
  gap: 4px;
  font-size: 13px;
  color: #444;
}

.course-form,
.course-form-empty {
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 14px;
}

.course-form h4 {
  margin: 0 0 12px 0;
}

.meta-row {
  font-size: 13px;
  margin-bottom: 8px;
}

.field {
  display: grid;
  gap: 4px;
  margin-bottom: 10px;
  font-size: 13px;
}

.form-actions {
  margin-top: 12px;
  display: flex;
  gap: 8px;
}

.alert {
  padding: 10px;
  border-radius: 4px;
  margin-top: 10px;
  font-size: 13px;
}

.alert-error {
  background-color: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.course-form-empty {
  color: #666;
  font-size: 14px;
}

@media (max-width: 1100px) {
  .courses-content {
    grid-template-columns: 1fr;
  }
}

@media (prefers-color-scheme: dark) {
  .courses-view {
    background-color: #2f2f2f;
  }

  .courses-view h3 {
    color: #f6f6f6;
  }

  .course-card {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .course-card.selected {
    border-color: #388bfd;
  }

  .course-header h4 {
    color: #5ba3ff;
  }

  .students-count {
    background-color: #1a3a4d;
    color: #7dc5ff;
  }

  .course-meta,
  .meta-row,
  .course-form-empty,
  .empty,
  .loading {
    color: #aaa;
  }

  .course-form,
  .course-form-empty {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .alert-error {
    background-color: #3d2c2c;
    color: #f8d7da;
    border-color: #5a4141;
  }
}
</style>
