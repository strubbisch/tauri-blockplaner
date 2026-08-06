<script setup lang="ts">
import { CourseGroupWithCount } from '../stores/appStore';

interface Props {
  courseGroups: CourseGroupWithCount[];
  isLoading?: boolean;
}

defineProps<Props>();
</script>

<template>
  <div class="course-groups-view">
    <h3>Kursgruppen</h3>
    
    <div v-if="isLoading" class="loading">Loading...</div>
    
    <div v-else-if="courseGroups.length === 0" class="empty">
      <p>Bisher keine Kursgruppen importiert.</p>
    </div>
    
    <div v-else class="groups-grid">
      <div v-for="group in courseGroups" :key="group.id" class="group-card">
        <div class="group-header">
          <h4>{{ group.subject }}</h4>
          <span class="group-type">{{ group.course_type }}</span>
        </div>
        <div class="students-count">Anzahl Schüler: {{ group.students_count }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.course-groups-view {
  padding: 20px;
  border-radius: 8px;
  background-color: #f9f9f9;
}

.course-groups-view h3 {
  margin-top: 0;
  color: #333;
}

.loading,
.empty {
  padding: 20px;
  text-align: center;
  color: #666;
  font-size: 14px;
}

.groups-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

.group-card {
  background: white;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.group-card:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.group-header h4 {
  margin: 0;
  color: #007bff;
  font-size: 15px;
}

.group-type {
  background-color: #e7f3ff;
  color: #0056b3;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.students-count {
  font-size: 12px;
  color: #999;
}

@media (prefers-color-scheme: dark) {
  .course-groups-view {
    background-color: #2f2f2f;
  }

  .course-groups-view h3 {
    color: #f6f6f6;
  }

  .group-card {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .group-header h4 {
    color: #5ba3ff;
  }

  .group-type {
    background-color: #1a3a4d;
    color: #7dc5ff;
  }

  .group-id,
  .empty,
  .loading {
    color: #aaa;
  }
}
</style>
