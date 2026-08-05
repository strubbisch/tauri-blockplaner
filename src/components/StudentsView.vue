<script setup lang="ts">
import type { Student, CourseGroup } from "../stores/appStore";

interface Props {
  students?: Student[];
  courseGroups?: CourseGroup[];
  isLoading?: boolean;
}

defineProps<Props>();

const getCourseName = (courseId: number, courseGroups: CourseGroup[] = []) => {
  const group = courseGroups.find(g => g.id === courseId);
  return group ? `${group.subject} (${group.course_type})` : `Course ${courseId}`;
};
</script>

<template>
  <div class="students-view">
    <h3>Students</h3>
    
    <div v-if="isLoading" class="loading">Loading...</div>
    
    <div v-else-if="!students || students.length === 0" class="empty">
      <p>No students imported yet</p>
    </div>
    
    <div v-else class="students-list">
      <div v-for="student in students" :key="student.id" class="student-card">
        <div class="student-header">
          <h4>{{ student.name }}</h4>
          <span class="student-id">ID: {{ student.id }}</span>
        </div>
        
        <div v-if="student.choices && student.choices.length > 0" class="student-choices">
          <div class="label">Choices:</div>
          <div class="choice-list">
            <span v-for="choiceId in student.choices.slice(0, 3)" :key="choiceId" class="choice-tag">
              {{ getCourseName(choiceId, courseGroups) }}
            </span>
            <span v-if="student.choices.length > 3" class="choice-tag more">
              +{{ student.choices.length - 3 }} more
            </span>
          </div>
        </div>
        
        <div v-if="student.courses && student.courses.length > 0" class="student-courses">
          <div class="label">Assigned:</div>
          <div class="course-list">
            <span v-for="courseId in student.courses" :key="courseId" class="course-tag">
              {{ getCourseName(courseId, courseGroups) }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.students-view {
  padding: 20px;
  border-radius: 8px;
  background-color: #f9f9f9;
}

.students-view h3 {
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

.students-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.student-card {
  background: white;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: all 0.2s ease;
}

.student-card:hover {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.student-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  border-bottom: 1px solid #eee;
  padding-bottom: 8px;
}

.student-header h4 {
  margin: 0;
  color: #333;
  font-size: 15px;
}

.student-id {
  font-size: 12px;
  color: #999;
}

.label {
  font-size: 12px;
  font-weight: 600;
  color: #666;
  margin-bottom: 5px;
  text-transform: uppercase;
}

.choice-list,
.course-list {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.choice-tag,
.course-tag {
  font-size: 12px;
  padding: 4px 8px;
  border-radius: 4px;
  background-color: #e7f3ff;
  color: #0056b3;
}

.course-tag {
  background-color: #d4edda;
  color: #155724;
}

.choice-tag.more {
  background-color: #f0f0f0;
  color: #666;
}

.student-choices,
.student-courses {
  margin-top: 8px;
}

@media (prefers-color-scheme: dark) {
  .students-view {
    background-color: #2f2f2f;
  }

  .students-view h3 {
    color: #f6f6f6;
  }

  .student-card {
    background-color: #1a1a1a;
    border-color: #444;
  }

  .student-header {
    border-bottom-color: #333;
  }

  .student-header h4 {
    color: #f6f6f6;
  }

  .student-id,
  .label,
  .empty,
  .loading {
    color: #aaa;
  }

  .choice-tag {
    background-color: #1a3a4d;
    color: #7dc5ff;
  }

  .course-tag {
    background-color: #1a3a1a;
    color: #7dff7d;
  }

  .choice-tag.more {
    background-color: #333;
    color: #ccc;
  }
}
</style>
