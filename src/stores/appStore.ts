import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export interface CourseGroup {
  id: number;
  subject: string;
  course_type: string;
}

export interface Student {
  id: number;
  name: string;
  choices: number[];
  courses: number[];
}

export interface AppData {
  course_groups: CourseGroup[];
  courses: unknown[];
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
    const result = await invoke<string>("import_xml", {
      filePath: filePath,
    });
    appData.value = JSON.parse(result);
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
    const result = await invoke<string>("import_json", {
      filePath: filePath,
    });
    appData.value = JSON.parse(result);
    return appData.value;
  } catch (err) {
    error.value = `Failed to import JSON: ${err}`;
    console.error(err);
    throw err;
  } finally {
    isLoading.value = false;
  }
};

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

export const useAppStore = () => ({
  appData,
  isLoading,
  error,
  fetchAppData,
  importXML,
  importJSON,
  clearData,
});
