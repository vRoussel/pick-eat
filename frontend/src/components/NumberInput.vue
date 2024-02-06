<script setup>

const model = defineModel({
    required: true,
    validator: (prop) => typeof prop === 'number' || prop === null,
})

const props = defineProps({
    min: {
        type: Number,
        default: null,
    },
    max: {
        type: Number,
        default: null,
    },
    step: {
        type: Number,
        default: 1,
    },
    placeholder: {
        type: String,
        default: '',
    },
    badvalue: {
        type: Boolean,
        default: false,
    },
});

function increment() {
    let new_value = model.value + props.step
    if (props.max != null && new_value > props.max) new_value = props.max
    model.value = new_value
}

function decrement() {
    let new_value = model.value - props.step
    if (props.min != null && new_value < props.min) new_value = props.min
    model.value = new_value
}
</script>

<template>
    <div class="w-32">
        <div class="flex flex-row w-full rounded-lg bg-transparent">
            <button type="button"
                class="bg-base text-base-content hover:bg-accent px-1 sm:px-2 rounded-l-md border-accent border-y border-l focus:!outline-none"
                :class="props.badvalue && '!input-error'" tabindex="-1" @click="decrement">
                <span class="m-auto text-2xl font-thin">−</span>
            </button>
            <input type="number" :placeholder="props.placeholder" step="any"
                class="input input-bordered rounded-none w-full min-w-[50px]" :class="props.badvalue && '!input-error'"
                :value="model" @input="(e) => model = e.target.value" />
            <button type="button"
                class="bg-base text-base-content hover:bg-accent px-1 sm:px-2 rounded-r-md border-accent border-y border-r focus:!outline-none"
                :class="props.badvalue && '!input-error'" tabindex="-1" @click="increment">
                <span class="m-auto text-2xl font-thin">+</span>
            </button>
        </div>
    </div>
</template>

