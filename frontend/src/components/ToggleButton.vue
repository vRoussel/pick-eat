<template>
  <button
    type="button"
    :class="buttonClass()"
    @click="toggle()"
  >
    <slot />
  </button>
</template>

<script>
export default {
    name: 'ToggleButton',
    props: {
        state: {
            type: Boolean
        }
    },
    emits: ['update:state'],
    methods: {
        buttonClass() {
            return {
                "btn": true,
                "btn-sm": true,
                "rounded-full": true,
                "btn-primary": this.state,
                "hover:btn-primary": true,
                "border-gray-300": true,
                "btn-outline": !this.state
            }
        },
        toggle() {
            this.$emit('update:state', !this.state)
        }
    },
}
</script>
