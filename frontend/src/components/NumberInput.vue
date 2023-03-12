<template>
    <div class="w-32 h-[42px]">
      <div class="flex flex-row h-full w-full rounded-lg bg-transparent">
        <button
          type="button"
          class="bg-base text-base-content hover:bg-base-200 h-full px-1 sm:px-2 rounded-l-md border-base-300 border-y border-l focus:!outline-none"
          :class="this.badvalue && '!input-error'"
          @click="decrement"
        >
          <span class="m-auto text-2xl font-thin">−</span>
        </button>
        <input
          type="number"
          :placeholder="placeholder"
          class="input input-bordered rounded-none w-full h-full min-w-[50px]"
          :class="this.badvalue && '!input-error'"
          :value="modelValue"
          @input="e => $emit('update:modelValue', parseInt(e.target.value))"
        >
        <button
          type="button"
          class="bg-base text-base-content hover:bg-base-200 h-full px-1 sm:px-2 rounded-r-md border-base-300 border-y border-r focus:!outline-none"
          :class="this.badvalue && '!input-error'"
          @click="increment"
        >
          <span class="m-auto text-2xl font-thin">+</span>
        </button>
      </div>
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
        },
        badvalue: {
            type: Boolean,
            default: false
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
