<template>
        <div class="column is-narrow">
            <button tabindex="-1" class="delete" type="button" @click="this.$emit('delete')"></button>
        </div>
        <span tabindex="-1" class="column has-text-right"> {{ ingredient_name }}</span>
        <input v-model.number="ingredient_quantity" class="input column is-2" min=0 step="any" type="number">
        <div class="column">
            <multiselect @keydown.ctrl.enter.prevent="new_unit($event.target.value)" @open="notify_input_selected" v-model="ingredient_unit" :options="searchable_units" label="full_name" searchable trackBy="searchable_name" valueProp="id" ref="multiselect"/>
        </div>
</template>

<script>
import Multiselect from '@vueform/multiselect'
import {obj_with_searchable_name} from '@/utils/utils.js'

export default {
    name: 'ingredient-list-item',
    inject: ["store"],
    components : {
        Multiselect
    },
    props: {
        id: Number,
        quantity: Number,
        unit_id: Number
    },
    computed: {
        ingredient_unit: {
            get() {
                return this.unit_id
            },
            set(v) {
                this.$emit('update:unit_id', v)
            },

        },
        ingredient_quantity: {
            get() {
                return this.quantity
            },
            set(v) {
                this.$emit('update:quantity', v == "" ? null : v)
            },

        },
        ingredient_name() {
            return this.store.state.ingredients.find(ingr => ingr.id === this.id).name
        },
        searchable_units() {
            return this.store.state.units.map(unit => obj_with_searchable_name(unit, "full_name"))
        },
    },
    methods: {
        new_unit(input) {
            this.$emit('newUnit', input)
        },
        notify_input_selected() {
            this.$emit('unit-input-selected', this.$refs.multiselect)
        }
    },
    emits: ['update:quantity', 'update:unit_id', 'delete', 'newUnit', 'unit-input-selected'],
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>


