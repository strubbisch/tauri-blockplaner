<script setup lang="ts">
import { useAppStore } from '../stores/appStore';


const appStore = useAppStore();

const handleSettingChange = async () => {
  if (appStore.appData.value) {
    await appStore.updateAppSettings(
      appStore.appData.value.number_of_blocks,
      appStore.appData.value.total_hours_per_week,
      appStore.appData.value.max_course_students_difference,
      appStore.appData.value.max_students_per_course,
    );
  }
};
</script>

<template>
    <div class="flex flex-col gap-4 rounded-lg bg-zinc-50 p-5">
        <h3 class="m-0 text-center text-xl font-semibold text-zinc-800">
            Einstellungen
        </h3>

        <div v-if="appStore.appData.value" class="grid grid-cols-[1fr_3fr_1fr] gap-4">            
                <div class="text-right">
                    Anzahl Blöcke:
                </div>
                <input v-model.number="appStore.appData.value.number_of_blocks" @change="handleSettingChange" type="range" min="20" max="40"/>
                <div>
                    {{ appStore.appData.value.number_of_blocks }}
                </div>            
                <div class="text-right">
                    max. Anzahl Stunden:
                </div>
                <input v-model.number="appStore.appData.value.total_hours_per_week" @change="handleSettingChange" type="range" min="32" max="50"/>
                <div>
                    {{ appStore.appData.value.total_hours_per_week }}
                </div>
                <div class="text-right">
                    max. Schülerdifferenz in Parallelkursen:
                </div>
                <input v-model.number="appStore.appData.value.max_course_students_difference" @change="handleSettingChange" type="range" min="0" max="10"/>
                <div>
                    {{ appStore.appData.value.max_course_students_difference }}
                </div>
                <div class="text-right">
                    max. Anzahl Schüler pro Kurs:
                </div>
                <input v-model.number="appStore.appData.value.max_students_per_course" @change="handleSettingChange" type="range" min="20" max="30"/>
                <div>
                    {{ appStore.appData.value.max_students_per_course }}
                </div>
            
        </div>
    </div>
</template>