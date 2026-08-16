<script setup lang="ts">
import { Block } from '../stores/appStore';


interface Props {
    blocks?: Block[];
    isLoading?: boolean; 
}

defineProps<Props>();

</script>

<template>
    <div class="rounded-lg bg-zinc-50 p-5">
        <h3 class="mt-0 text-xl font-semibold text-zinc-800">Blöcke</h3>

        <div v-if="isLoading" class="p-5 text-center text-sm text-zinc-600">Loading...</div>
    
        <div v-else-if="!blocks || blocks.length === 0" class="p-5 text-center text-sm text-zinc-600">
            <p>Bisher keine Blockung berechnet.</p>
        </div>

        <div v-else class="flex flex-col gap-3">
            <div
                v-for="block in blocks"
                :key="block.id"
                class="rounded-md border border-zinc-300 bg-white p-3 shadow-sm transition hover:shadow-md"
            >
                <div class="mb-2.5 flex items-center justify-between border-b border-zinc-200 pb-2">
                    <h4 class="m-0 text-sm font-semibold text-zinc-800">Block {{ block.number }} ({{ block.length === 'Double' ? 2 : 1 }}h)</h4>
                </div>
            </div>
        </div>
    </div>
</template>