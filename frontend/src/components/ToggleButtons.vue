<template>
  <div class="buttons flex gap-2 flex-wrap">
    <template
      v-for="el in choices"
      :key="el.id"
    >
      <button
        type="button"
        :class="buttonClass(el)"
        @click="toggle(el)"
      >
        {{ el.name }}
      </button>
    </template>
    <button
      v-if="extendModalComponent"
      class="btn rounded-full btn-primary btn-outline btn-sm"
      @click="this.$refs.modal.open()"
      type="button"
    >+</button>
  </div>
  <component
    :is="extendModalComponent"
    ref="modal"
    @created="created"
  />
</template>

<script>
export default {
    name: 'ToggleButtons',
    components : {
    },
    props: {
        choices: {
            type: Array
        },
        picked: {
            type: Set
        },
        extendable: {
            type: Boolean
        },
        extendModalComponent : {
            type: Object
        },
    },
    emits: ['update:picked'],
    methods: {
        buttonClass(el) {
            return {
                "btn": true,
                "btn-sm": true,
                "rounded-full": true,
                "btn-primary": this.picked.has(el.id),
                "hover:btn-primary": true,
                "border-gray-300": true,
                "btn-outline": !this.picked.has(el.id),
            }
        },
        toggle(el) {
            if (this.picked.has(el.id))
                this.picked.delete(el.id)
            else
                this.picked.add(el.id)
            this.$emit('update:picked', this.picked)
        },
        created(new_choice) {
            this.toggle(new_choice)
        }
    },
}
</script>
