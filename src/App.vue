<script setup lang="ts">
import { ref } from "vue";
import ImportComponent from "./components/ImportComponent.vue";
import CourseGroupsView from "./components/CourseGroupsView.vue";
import CoursesView from "./components/CoursesView.vue";
import { useAppStore } from "./stores/appStore";
import SettingsView from "./components/SettingsView.vue";
import StudentsView from "./components/StudentsView.vue";
import CourseBlockView from "./components/CourseBlockView.vue";
import BlocksView from "./components/BlocksView.vue";

const appStore = useAppStore();
const activeView = ref<"overview" | "courses" | "settings" | "students" | "courseblocks" | "blocks">("overview");

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
        <button
          :class="[
            'btn',
            activeView === 'courseblocks'
              ? 'btn-primary'
              : 'btn-secondary'
          ]"
          @click="activeView = 'courseblocks'"
        >
          Kursblöcke
        </button>
        <button
          :class="[
            'btn',
            activeView === 'students'
              ? 'btn-primary'
              : 'btn-secondary'
          ]"
          @click="activeView = 'students'"
        >
          Schüler
        </button>
        <button
          :class="[
            'btn',
            activeView === 'blocks'
              ? 'btn-primary'
              : 'btn-secondary'
          ]"
          @click="activeView = 'blocks'"
        >
          Blöcke
        </button>
        <button
          :class="[
            'btn',
            activeView === 'settings'
              ? 'btn-primary'
              : 'btn-secondary',
          ]"
          @click="activeView = 'settings'"
        >
          Einstellungen
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
      <SettingsView
        v-else-if="activeView === 'settings'"
        class="flex-1"
      />
      <StudentsView
        v-else-if="activeView === 'students'"
        class="flex-1"
        :students="appStore.appData.value.students"
        :course-groups="appStore.appData.value.course_groups"
        :courses="appStore.appData.value.courses"
        :is-loading="appStore.isLoading.value"
      />
      <CourseBlockView
        v-else-if="activeView === 'courseblocks'"
        class="flex-1"
        :course-blocks="appStore.appData.value.course_blocks"
        :course-groups="appStore.appData.value.course_groups"
        :courses="appStore.appData.value.courses"
      />
      <BlocksView
        v-else-if="activeView === 'blocks'"
        class="flex-1"
        :blocks="appStore.appData.value.blocks"
        :app-data="appStore.appData.value"
        :is-loading="appStore.isLoading.value"
      />
    </div>
  </main>
</template>
