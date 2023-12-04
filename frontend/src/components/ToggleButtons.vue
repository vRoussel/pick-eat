<template>
    <div class="buttons flex gap-2 flex-wrap">
        <template v-for="el in choices" :key="el.id">
            <toggle-button :state="picked.has(el.id)" v-bind="$attrs" @update:state="toggle(el.id)">
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

<script>
import ToggleButton from '@/components/ToggleButton.vue'
export default {
    name: 'ToggleButtons',
    components: {
        ToggleButton,
    },
    props: {
        choices: {
            type: Array,
        },
        picked: {
            type: Set,
        },
        extendable: {
            type: Boolean,
        },
        extendModalComponent: {
            type: Object,
        },
    },
    emits: ['update:picked'],
    computed: {
        _picked() {
            return new Set(this.picked)
        },
    },
    methods: {
        toggle(id) {
            if (this._picked.has(id)) this._picked.delete(id)
            else this._picked.add(id)
            this.$emit('update:picked', this._picked)
        },
        created(new_choice) {
            this.toggle(new_choice.id)
        },
    },
}
</script>
