import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface CourseGroup {
  id: number;
  subject: string;
  course_type: string;
}

export interface CourseGroupWithCount extends CourseGroup {
  students_count: number;
}

export interface Student {
  id: number;
  name: string;
  choices: number[];
  courses: number[];
}

export interface Course {
  id: number;
  course_group: number;
  number: number;
  teacher: string;
  room: string;
  hours_per_week: number;
  students: number[];
}

export interface AppData {
  course_groups: CourseGroup[];
  courses: Course[];
  students: Student[];
  blocks: unknown[];
  course_units: unknown[];
  block_assignments: unknown[];
  course_blocks: unknown[];
  number_of_blocks: number;
  total_hours_per_week: number;
  max_course_students_difference: number;
  max_students_per_course: number;
}

const appData = ref<AppData | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);

const courseGroups = computed<CourseGroupWithCount[]>(() => {
  if (!appData.value) return [];
  const countById = new Map<number, number>();
  for (const student of appData.value.students) {
    for (const choice of student.choices) {
      countById.set(choice, (countById.get(choice) ?? 0) + 1);
    }
  }
  return appData.value.course_groups.map(g => ({
    ...g,
    students_count: countById.get(g.id) ?? 0,
  }));
});

const fetchAppData = async () => {
  try {
    isLoading.value = true;
    error.value = null;
    const result = await invoke<string>("get_app_data");
    appData.value = JSON.parse(result);
  } catch (err) {
    error.value = `Failed to fetch app data: ${err}`;
    console.error(err);
  } finally {
    isLoading.value = false;
  }
};

const importXML = async (filePath: string) => {
  try {
    isLoading.value = true;
    error.value = null;
    appData.value = await invoke<AppData>("import_xml", {
      filePath: filePath,
    });
    return appData.value;
  } catch (err) {
    error.value = `Failed to import XML: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
};

const importJSON = async (filePath: string) => {
  try {
    isLoading.value = true;
    error.value = null;
    appData.value = await invoke<AppData>("import_json", {
      filePath: filePath,
    });
    return appData.value;
  } catch (err) {
    error.value = `Failed to import JSON: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
};

const exportJSON = async (filePath: String) => {
  try {
    isLoading.value = true;
    error.value = null;
    console.log("[DEBUG] Starting export to:", filePath);
    const result = await invoke("export_json", {
      filePath: filePath,
    });
    console.log("[DEBUG] Export completed successfully, result:", result);
  } catch (err) {
    console.error("[ERROR] Export failed:", err);
    error.value = `${err}`;
    throw err;
  } finally {
    isLoading.value = false;
  }
}

const updateAppSettings = async (
  numberofBlocks?: number,
  totalHoursPerWeek?: number,
  maxCourseStudentsDifference?: number,
  maxStudentsPerCourse?: number,
) => {
  try {
    isLoading.value = true;
    error.value = null;
    await invoke("update_app_settings", {
      numberOfBlocks: numberofBlocks,
      totalHoursPerWeek: totalHoursPerWeek,
      maxCourseStudentsDifference: maxCourseStudentsDifference,
      maxStudentsPerCourse: maxStudentsPerCourse,
    });
  } catch (err) {
    error.value = `Failed to update settings: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
}

const clearData = async () => {
  try {
    isLoading.value = true;
    error.value = null;
    await invoke("clear_app_data");
    appData.value = null;
  } catch (err) {
    error.value = `Failed to clear data: ${err}`;
    console.error(err);
  } finally {
    isLoading.value = false;
  }
};

const updateCourse = async (
  courseId: number,
  teacher: string,
  room: string,
  hoursPerWeek: number,
) => {
  try {
    isLoading.value = true;
    error.value = null;
    appData.value = await invoke<AppData>("update_course", {
      courseId,
      teacher,
      room,
      hoursPerWeek,
    });
    return appData.value;
  } catch (err) {
    error.value = `Failed to update course: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
};

const createParallelCourse = async (
  courseGroupId: number,
  hoursPerWeek: number,
) => {
  try {
    isLoading.value = true;
    error.value = null;
    appData.value = await invoke<AppData>("create_parallel_course", {
      courseGroupId,
      hoursPerWeek,
    });
    return appData.value;
  } catch (err) {
    error.value = `Fehler beim Erstellen des Parallelkurses: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
}

const deleteCourse = async (
  courseGroupId: number,
) => {
  try {
    isLoading.value = true;
    error.value = null;
    appData.value = await invoke<AppData>("delete_course", {
      courseGroupId,
    });
    return appData.value;
  } catch(err) {
    error.value = `Fehler beim Löschen des Kurses: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
}

const createCourses = async () => {
  try {
    isLoading.value = true;
    error.value = null;
    appData.value = await invoke<AppData>("create_courses");
    console.log(appData.value);
    return appData.value;
  } catch (err) {
    error.value =`Fehler beim Erstellen der Kurse: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
}

export const useAppStore = () => ({
  appData,
  courseGroups,
  isLoading,
  error,
  fetchAppData,
  importXML,
  importJSON,
  clearData,
  updateCourse,
  createParallelCourse,
  deleteCourse,
  createCourses,
  exportJSON,
  updateAppSettings,
});
