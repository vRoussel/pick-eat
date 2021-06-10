<template>
    <div class="level is-mobile mb-2">
        <div class="level-left">
        <button type="button" class="button is-rounded is-info is-outlined is-small level-item" @mousedown="save_ingredient_search" @click="this.$emit('newIngredient', this.ingredient_search)">Ingrédient manquant ?</button>
        <button type="button" class="button is-rounded is-info is-outlined is-small level-item" @click="this.$emit('newUnit', null)">Unité manquante ?</button>
        </div>
    </div>
    <Multiselect class="mb-4" mode="multiple" :options="ingr_remaining" label="name" searchable @select="add_ingr" trackBy="searchable_name" object valueProp="id" v-model="dummy" ref="multiselect"/>
    <div class="columns is-vcentered is-mobile" v-for="ingr in picked.values()" :key="ingr.id">
        <ingredient-list-item @delete="del_ingr(ingr.id)" v-model:quantity="ingr.quantity" :id="ingr.id" v-model:unit_id="ingr.unit_id"></ingredient-list-item>
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
            ingredient_search: null
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
    emits: ['update:picked', 'newIngredient'],
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
        }
    }
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
