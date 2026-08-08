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
  <main class="container">
    <ImportComponent v-if="!appStore.appData.value" />

    <div v-else class="data-views">
      <nav class="view-menu" aria-label="Ansichten">
        <button
          class="view-menu-button"
          :class="{ active: activeView === 'overview' }"
          @click="activeView = 'overview'"
        >
          Übersicht
        </button>
        <button
          class="view-menu-button"
          :class="{ active: activeView === 'courses' }"
          @click="activeView = 'courses'"
        >
          Kurse
        </button>
      </nav>

      <div v-if="activeView === 'overview'" class="overview-layout">
        <ImportComponent />
        <CourseGroupsView
          :course-groups="appStore.courseGroups.value"
          :is-loading="appStore.isLoading.value"
        />
      </div>
      <CoursesView
        v-else-if="activeView === 'courses'"
        class="courses-view-panel"
        :courses="appStore.appData.value.courses"
        :course-groups="appStore.appData.value.course_groups"
        :is-loading="appStore.isLoading.value"
      />
    </div>
  </main>
</template>

<style scoped>
.header {
  text-align: center;
  margin-bottom: 30px;
}

.header h1 {
  margin: 0;
  color: #0f0f0f;
}

.subtitle {
  margin: 5px 0 0 0;
  font-size: 16px;
  color: #666;
}

.data-views {
  gap: 20px;
  width: 100%;
}

.overview-layout {
  display: flex;
  gap: 20px;
}

.overview-layout > * {
  flex: 1;
}

.courses-view-panel {
  flex: 1;
}

.view-menu {
  display: flex;
  gap: 10px;
  justify-content: center;
}

.view-menu-button {
  border: 1px solid #ddd;
}

.view-menu-button.active {
  background-color: #007bff;
  color: #fff;
  border-color: #0056b3;
}

.section {
  margin: 30px 0;
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
  background-color: #f9f9f9;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.3s ease;
  font-weight: 500;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background-color: #5a6268;
}

@media (prefers-color-scheme: dark) {
  .view-menu-button {
    border-color: #444;
  }

  .view-menu-button.active {
    background-color: #1f6feb;
    border-color: #388bfd;
  }

  .header h1 {
    color: #f6f6f6;
  }

  .subtitle {
    color: #aaa;
  }

  .section {
    background-color: #2f2f2f;
    border-color: #444;
  }

  .section h2 {
    color: #f6f6f6;
  }

  .section p {
    color: #ccc;
  }
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding: 20px;
  margin: 0 auto;
  display: flex;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1,
h2,
h3 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover:not(:disabled) {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

button:disabled {
  cursor: not-allowed;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
