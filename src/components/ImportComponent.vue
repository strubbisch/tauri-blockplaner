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
  <div class="import-container">
    <h2>{{ appStore.appData.value ? "Daten importiert" : "Daten importieren" }}</h2>

    <div class="import-buttons" v-if="!appStore.appData.value">
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

    <div v-if="appStore.error.value" class="alert alert-error">
      {{ appStore.error.value }}
    </div>

    <div v-if="appStore.appData.value && courseGroupsCount > 0" class="data-summary">
      <div class="summary-grid">
        <div class="summary-item">
          <strong>Kursgruppen</strong>
          <div class="count">{{ courseGroupsCount }}</div>
        </div>
        <div class="summary-item">
          <strong>Kurse</strong>
          <div class="count">{{ coursesCount }}</div>
        </div>
        <div class="summary-item">
          <strong>Schüler</strong>
          <div class="count">{{ studentsCount }}</div>
        </div>
        <div class="summary-item">
          <strong>Blöcke</strong>
          <div class="count">{{  blocksCount }}</div>
        </div>
      </div>
      <button @click="clearData" class="btn btn-secondary">
        Lösche alle Daten
      </button>
    </div>
  </div>
</template>

<style scoped>
.import-container {
  padding: 20px;
  border: 1px solid #ddd;
  border-radius: 8px;
  margin: 20px 0;
  background-color: #f9f9f9;
  flex: 1;
}

.import-container h2 {
  margin-top: 0;
  color: #333;
}

.import-buttons {
  display: flex;
  gap: 10px;
  margin: 15px 0;
  flex-wrap: wrap;
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

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background-color: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #0056b3;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
  margin-top: 10px;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #5a6268;
}

.alert {
  padding: 12px;
  border-radius: 4px;
  margin: 10px 0;
  font-size: 14px;
}

.alert-error {
  background-color: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.data-summary {
  margin-top: 20px;
  padding: 15px;
  background-color: white;
  border: 2px solid #d4edda;
  border-radius: 4px;
}

.data-summary h3 {
  margin-top: 0;
  color: #155724;
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 15px;
  margin-bottom: 15px;
}

.summary-item {
  text-align: center;
}

.summary-item strong {
  display: block;
  font-size: 12px;
  color: #555;
  text-transform: uppercase;
  margin-bottom: 5px;
}

.count {
  font-size: 28px;
  font-weight: bold;
  color: #007bff;
}

@media (prefers-color-scheme: dark) {
  .import-container {
    background-color: #2f2f2f;
    border-color: #444;
  }

  .import-container h2 {
    color: #f6f6f6;
  }

  .data-summary {
    background-color: #1a1a1a;
    border-color: #1a3a1a;
  }

  .data-summary h3 {
    color: #7dff7d;
  }

  .summary-item strong {
    color: #aaa;
  }

  .count {
    color: #5ba3ff;
  }

  .alert-error {
    background-color: #3d2c2c;
    color: #f8d7da;
    border-color: #5a4141;
  }
}
</style>
