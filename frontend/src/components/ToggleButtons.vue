<script setup>
import { computed } from 'vue'

import ToggleButton from '@/components/ToggleButton.vue'

const model = defineModel('picked', {
    type: Set,
    required: true,
})

const props = defineProps({
    choices: {
        type: Array,
    },
    extendable: {
        type: Boolean,
    },
    extendModalComponent: {
        type: Object,
    },
})

const _picked = computed(() => {
    return new Set(model.value)
})

function toggle(id) {
    if (_picked.value.has(id)) _picked.value.delete(id)
    else _picked.value.add(id)
    model.value = _picked.value
}

function created(new_choice) {
    toggle(new_choice.id)
}
</script>

<template>
    <div class="buttons flex gap-2 flex-wrap">
        <template v-for="el in choices" :key="el.id">
            <toggle-button :state="model.has(el.id)" v-bind="$attrs" @update:state="toggle(el.id)">
                {{ el.name }}
            </toggle-button>
        </template>
        <button
            v-if="extendable && extendModalComponent"
            class="btn rounded-full btn-primary btn-outline btn-sm"
            type="button"
            @click="$refs.modal.open()"
        >
            +
        </button>
    </div>
    <component :is="extendModalComponent" ref="modal" @created="created" />
</template>
