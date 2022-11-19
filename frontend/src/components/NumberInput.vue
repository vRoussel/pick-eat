<template>
  <div class="flex flex-row h-10 w-full rounded-lg bg-transparent">
    <button
      type="button"
      class="bg-accent text-accent-content hover:bg-accent-focus h-full px-1 sm:px-2 rounded-l-md cursor-pointer outline-none"
      @click="decrement"
    >
      <span class="m-auto text-2xl font-thin">−</span>
    </button>
    <input
      type="number"
      :placeholder="placeholder"
      class="input input-bordered rounded-none w-full h-full min-w-[50px]"
      :value="modelValue"
      @input="e => $emit('update:modelValue', parseInt(e.target.value))"
    >
    <button
      type="button"
      class="bg-accent text-accent-content hover:bg-accent-focus h-full px-1 sm:px-2 rounded-r-md cursor-pointer"
      @click="increment"
    >
      <span class="m-auto text-2xl font-thin">+</span>
    </button>
  </div>
</template>

<script>
export default {
    name: 'NumberInput',
    props: {
        min: {
            type: Number,
            default: null
        },
        max: {
            type: Number,
            default: null
        },
        step: {
            type: Number,
            default: 1
        },
        modelValue: {
            //type: Number,
            required: true,
            validator: prop => typeof prop === 'number' || prop === null
        },
        placeholder: {
            type: String,
            default: ""
        }
    },
    emits: ['update:modelValue'],
    methods: {
        increment() {
            let new_value = this.modelValue + this.step
            if (this.max != null && new_value > this.max)
                new_value = this.max
            this.$emit('update:modelValue', new_value)
        },
        decrement() {
            let new_value = this.modelValue - this.step
            if (this.min != null && new_value < this.min)
                new_value = this.min
            this.$emit('update:modelValue', new_value)
        }
    }
}
</script>
