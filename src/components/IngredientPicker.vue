<template>
    <div class="level is-mobile mb-2">
        <div class="level-left">
        <button type="button" class="button is-rounded is-info is-outlined is-small level-item" @mousedown="save_ingredient_search" @click="new_ingredient(ingredient_search)">Ingrédient manquant ?</button>
        <button type="button" class="button is-rounded is-info is-outlined is-small level-item" @mousedown="save_unit_search" @click="new_unit(unit_search)">Unité manquante ?</button>
        </div>
    </div>
    <Multiselect @keydown.ctrl.enter.prevent="new_ingredient($event.target.value)" class="mb-4" mode="multiple" :options="ingr_remaining" label="name" searchable @select="add_ingr" trackBy="searchable_name" object valueProp="id" v-model="dummy" ref="multiselect"/>
    <div class="columns is-vcentered is-mobile" v-for="ingr in picked.values()" :key="ingr.id">
        <ingredient-list-item @delete="del_ingr(ingr.id)" v-model:quantity="ingr.quantity" :id="ingr.id" v-model:unit_id="ingr.unit_id" @newUnit="new_unit" @unit-input-selected="save_current_unit_input"></ingredient-list-item>
    </div>
</template>

<script>
import Multiselect from '@vueform/multiselect'
import IngredientListItem from '@/components/IngredientListItem.vue'
import {obj_with_searchable_name} from '@/utils/utils.js'

export default {
    name: 'ingredient-picker',
    inject: ["store"],
    components : {
        Multiselect,
        IngredientListItem
    },
    props: {
        picked: {
            type: Map
        }
    },
    data: function() {
        return {
            dummy: null,
            ingredient_search: null,
            unit_search: null,
            current_unit_input: null
        }
    },
    computed: {
        picked_obj() {
            return Object.fromEntries(this.picked);
        },
        ingr_remaining() {
            return this.store.state.ingredients
                .filter(ingr => !this.picked.has(ingr.id))
                .map(ingr => obj_with_searchable_name(ingr, "name"))
        },
    },
    emits: ['update:picked', 'newIngredient', 'newUnit'],
    methods: {
        add_ingr(ingr) {
            this.picked.set(ingr.id, {
                id: ingr.id,
                unit_id: ingr.default_unit ? ingr.default_unit.id : null,
                quantity: null,
            })
            this.$emit('update:picked', this.picked)
            this.$refs.multiselect.clear()
        },
        del_ingr(id) {
            this.picked.delete(id)
            this.$emit('update:picked', this.picked)
        },
        save_ingredient_search() {
            this.ingredient_search = this.$refs.multiselect.search
        },
        save_unit_search() {
            if (this.current_unit_input !== undefined)
                this.unit_search = this.current_unit_input.search
        },
        save_current_unit_input(elem) {
            this.current_unit_input = elem
        },
        new_ingredient(input) {
            this.$emit('newIngredient', input)
        },
        new_unit(input) {
            this.$emit('newUnit', input)
        }
    }
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
