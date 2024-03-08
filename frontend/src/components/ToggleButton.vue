<script setup>
const model = defineModel('state', {
    type: Boolean,
    required: true,
})

function buttonClass() {
    return {
        btn: true,
        'btn-sm': true,
        'rounded-full': true,
        'btn-primary': model.value,
        'hover:btn-primary': true,
        'border-gray-300': true,
        'btn-outline': !model.value,
    }
}

function toggle() {
    model.value = !model.value
}
</script>

<template>
    <button type="button" :class="buttonClass()" @click="toggle()">
        <slot />
    </button>
</template>
