<script setup>
const props = defineProps({
    error_queue: Array,
    info_queue: Array
})
</script>

<template>
    <div class="toast">
        <div v-for="msg in props.error_queue" :key="msg" class="alert alert-error">
            <div>
                <span>{{ msg }}</span>
            </div>
        </div>
        <div v-for="msg in props.info_queue" :key="msg" class="alert alert-info">
            <div>
                <span>{{ msg }}</span>
            </div>
        </div>
    </div>
</template>
