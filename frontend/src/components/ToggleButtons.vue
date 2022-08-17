<template>
    <div class="buttons flex gap-2 flex-wrap">
        <template v-for="el in choices" :key="el.id">
            <button type="button" :class="buttonClass(el)" @click="toggle(el)">{{ el.name }}</button>
        </template>
        <label v-if="extendModalComponent" :for="modal_id" class="btn rounded-full btn-primary btn-outline btn-sm">+</label>
    </div>
    <component :modal_id="modal_id" :is="extendModalComponent" @created="created"></component>
</template>

<script>
import {random_str} from '@/utils/utils.js'

export default {
    name: 'toggle-buttons',
    components : {
    },
    data: function() {
        return {
            modal_id: random_str(5)
        }
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
