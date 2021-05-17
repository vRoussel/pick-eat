<template>
        <div class="column is-narrow">
            <button class="delete" type="button" @click="this.$emit('delete')"></button>
        </div>
        <span class="column has-text-right"> {{ ingredient_name }}</span>
        <input v-model.number="ingredient_quantity" class="input column is-2" min=0 step="any" type="number">
        <div class="column">
            <multiselect v-model="ingredient_unit" :options="store.state.units" label="full_name" searchable trackBy="full_name" valueProp="id"/>
        </div>
</template>

<script>
import Multiselect from '@vueform/multiselect'

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
    },
    emits: ['update:quantity', 'update:unit_id', 'delete'],
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>


