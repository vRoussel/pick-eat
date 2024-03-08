<script setup>
const props = defineProps({
    error_queue: Array,
    info_queue: Array,
})
</script>

<template>
    <div class="toast">
        <div v-for="msg in props.error_queue" :key="msg" class="alert alert-error">
            <p class="whitespace-normal">{{ msg }}</p>
        </div>
        <div v-for="msg in props.info_queue" :key="msg" class="alert alert-info">
            <p class="whitespace-normal">{{ msg }}</p>
        </div>
    </div>
</template>
