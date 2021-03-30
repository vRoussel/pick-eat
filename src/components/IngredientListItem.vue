<template>
        <div class="column is-narrow">
            <button class="delete" type="button" @click="this.$emit('delete')"></button>
        </div>
        <span class="column has-text-right"> {{ ingredient_name }}</span>
        <input v-model.number="ingredient_quantity" class="input column is-2" min=0 step="any" type="number">
        <div class="column">
            <multiselect v-model="ingredient_unit" :options="store.units" label="full_name" track-by="id" placeholder="Unité"></multiselect>
        </div>
</template>

<script>
import store from '@/store/store.js'
import Multiselect from '@suadelabs/vue3-multiselect'

export default {
    name: 'ingredient-list-item',
    components : {
        Multiselect
    },
    //TODO add types
    props: ['id', 'quantity', 'unit_id'],
    data: function() {
        return {
            store: store
        }
    },
    computed: {
        ingredient_unit: {
            get() {
                if (this.unit_id == null)
                    return null
                else
                    return this.store.units.find(unit => unit.id === this.unit_id)
            },
            set(v) {
                this.$emit('update:unit_id', v == null ? null : v.id)
            },

        },
        ingredient_quantity: {
            get() {
                return this.quantity
            },
            set(v) {
                this.$emit('update:quantity', v)
            },

        },
        ingredient_name() {
            return this.store.ingredients.find(ingr => ingr.id === this.id).name
        },
    },
    emits: ['update:quantity', 'update:unit_id', 'delete'],
}
</script>

<style src="@suadelabs/vue3-multiselect/dist/vue3-multiselect.css"></style>


