<script setup lang="ts">
import { computed } from "vue";
import { open, ask } from "@tauri-apps/plugin-dialog";
import { useAppStore } from "../stores/appStore";

const appStore = useAppStore();

const courseGroupsCount = computed(() => 
  appStore.appData.value?.course_groups?.length || 0
);
const studentsCount = computed(() => 
  appStore.appData.value?.students?.length || 0
);

const coursesCount = computed(() => 
  appStore.appData.value?.courses?.length || 0
);

const blocksCount = computed(() => 
  appStore.appData.value?.blocks?.length || 0
);

async function importXML() {
  try {
    console.log("Opening XML file dialog...");
    const selectedPath = await open({
      multiple: false,
      filters: [
        {
          name: "XML Files",
          extensions: ["xml"],
        },
      ],
    });

    console.log("Selected path:", selectedPath);

    if (!selectedPath) {
      console.log("No file selected");
      return;
    }

    const filePath = Array.isArray(selectedPath) ? selectedPath[0] : selectedPath;
    console.log("Importing XML from:", filePath);
    await appStore.importXML(filePath);
    console.log("XML import successful");
  } catch (error) {
    console.error("Error during XML import:", error);
  }
}

async function importJSON() {
  try {
    console.log("Opening JSON file dialog...");
    const selectedPath = await open({
      multiple: false,
      filters: [
        {
          name: "JSON Files",
          extensions: ["json"],
        },
      ],
    });

    console.log("Selected path:", selectedPath);

    if (!selectedPath) {
      console.log("No file selected");
      return;
    }

    const filePath = Array.isArray(selectedPath) ? selectedPath[0] : selectedPath;
    console.log("Importing JSON from:", filePath);
    await appStore.importJSON(filePath);
    console.log("JSON import successful");
  } catch (error) {
    console.error("Error during JSON import:", error);
  }
}

async function clearData() {
  const confirmed = await ask("Wirklich alle Daten löschen?", "Blockplaner");
  if (confirmed) {
    await appStore.clearData();
  }
}
</script>

<template>
  <div class="my-5 flex-1 rounded-lg border border-zinc-300 bg-zinc-50 p-5">
    <h2 class="mt-0 text-center text-2xl font-semibold text-zinc-800">
      {{ appStore.appData.value ? "Daten importiert" : "Daten importieren" }}
    </h2>

    <div v-if="!appStore.appData.value" class="my-4 flex flex-wrap gap-2.5">
      <button
        @click="importXML"
        :disabled="appStore.isLoading.value"
        class="btn btn-primary"
      >
        {{ appStore.isLoading.value ? "Importiere..." : "Import XML" }}
      </button>
      <button
        @click="importJSON"
        :disabled="appStore.isLoading.value"
        class="btn btn-primary"
      >
        {{ appStore.isLoading.value ? "Importiere..." : "Import JSON" }}
      </button>
    </div>

    <div
      v-if="appStore.error.value"
      class="my-2.5 rounded border border-red-300 bg-red-100 p-3 text-sm text-red-900"
    >
      {{ appStore.error.value }}
    </div>

    <div
      v-if="appStore.appData.value && courseGroupsCount > 0"
      class="mt-5 rounded border-2 border-green-200 bg-white p-4"
    >
      <div class="mb-4 grid grid-cols-[repeat(auto-fit,minmax(120px,1fr))] gap-4">
        <div class="text-center">
          <strong>Kursgruppen</strong>
          <div class="text-3xl font-bold text-blue-500">{{ courseGroupsCount }}</div>
        </div>
        <div class="text-center">
          <strong>Kurse</strong>
          <div class="text-3xl font-bold text-blue-500">{{ coursesCount }}</div>
        </div>
        <div class="text-center">
          <strong>Schüler</strong>
          <div class="text-3xl font-bold text-blue-500">{{ studentsCount }}</div>
        </div>
        <div class="text-center">
          <strong>Blöcke</strong>
          <div class="text-3xl font-bold text-blue-500">{{ blocksCount }}</div>
        </div>
      </div>
      <button
        @click="clearData"
        class="mt-2.5 cursor-pointer rounded border border-zinc-600 bg-zinc-500 px-5 py-2.5 text-sm font-medium text-white shadow transition hover:bg-zinc-600"
      >
        Lösche alle Daten
      </button>
    </div>
  </div>
</template>
