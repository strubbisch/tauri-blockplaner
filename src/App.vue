<script setup lang="ts">
import { ref } from "vue";
import ImportComponent from "./components/ImportComponent.vue";
import CourseGroupsView from "./components/CourseGroupsView.vue";
import CoursesView from "./components/CoursesView.vue";
import { useAppStore } from "./stores/appStore";

const appStore = useAppStore();
const activeView = ref<"overview" | "courses">("overview");

</script>

<template>
  <main class="mx-auto flex w-full p-5">
    <ImportComponent v-if="!appStore.appData.value" />

    <div v-else class="flex w-full flex-col gap-5">
      <nav class="flex justify-center gap-2.5" aria-label="Ansichten">
        <button
          :class="[
            'btn',
            activeView === 'overview'
              ? 'btn-primary'
              : 'btn-secondary',
          ]"
          @click="activeView = 'overview'"
        >
          Übersicht
        </button>
        <button
          :class="[
            'btn',
            activeView === 'courses'
              ? 'btn-primary'
              : 'btn-secondary',
          ]"
          @click="activeView = 'courses'"
        >
          Kurse
        </button>
      </nav>

      <div v-if="activeView === 'overview'" class="flex gap-5">
        <ImportComponent class="flex-1" />
        <CourseGroupsView
          class="flex-1"
          :course-groups="appStore.courseGroups.value"
          :is-loading="appStore.isLoading.value"
        />
      </div>
      <CoursesView
        v-else-if="activeView === 'courses'"
        class="flex-1"
        :courses="appStore.appData.value.courses"
        :course-groups="appStore.appData.value.course_groups"
        :is-loading="appStore.isLoading.value"
      />
    </div>
  </main>
</template>
