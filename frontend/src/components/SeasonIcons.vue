<script setup>
import { inject, computed } from 'vue'

const icons = inject('icons')
const props = defineProps({
    seasons: Array,
})

const season_labels = computed(() => {
    return props.seasons.map((r) => r.label)
})

function dynClass(label) {
    return {
        'text-primary': has_season(label),
        'text-base-content': !has_season(label),
        'opacity-15': !has_season(label),
    }
}

function has_season(label) {
    return season_labels.value.includes(label)
}
</script>

<template>
    <div class="flex">
        <span class="icon">
            <Icon :icon="icons.flower" :inline="true" :class="dynClass('spring')" />
        </span>
        <span class="icon">
            <Icon :icon="icons.sun" :inline="true" :class="dynClass('summer')" />
        </span>
        <span class="icon">
            <Icon :icon="icons.leaf" :inline="true" :class="dynClass('fall')" />
        </span>
        <span class="icon">
            <Icon :icon="icons.snowflake" :inline="true" :class="dynClass('winter')" />
        </span>
    </div>
</template>
