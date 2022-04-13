<template>
    <div class="level is-mobile mb-2">
        <div class="level-left">
        <button type="button" class="button is-rounded is-primary is-outlined is-small level-item" @mousedown="save_ingredient_search" @click="open_ingr_modal">Ingrédient manquant ?</button>
        <button type="button" class="button is-rounded is-primary is-outlined is-small level-item" @mousedown="save_unit_search" @click="open_unit_modal">Unité manquante ?</button>
        </div>
    </div>
    <Multiselect @keydown.ctrl.enter.prevent="save_ingredient_search(), open_ingr_modal()" class="mb-4" mode="multiple" :options="ingr_remaining" label="name" searchable @select="pick_ingr" trackBy="searchable_name" object valueProp="id" v-model="dummy" ref="multiselect"/>
    <div class="columns is-vcentered is-mobile" v-for="ingr in picked.values()" :key="ingr.id">
        <ingredient-list-item @delete="del_ingr(ingr.id)" v-model:quantity="ingr.quantity" :id="ingr.id" v-model:unit_id="ingr.unit_id" @createUnit="save_unit_search(), open_unit_modal()" @unit-input-selected="save_current_unit_input"></ingredient-list-item>
    </div>
    <dynamic-modal ref="modal_ingr">
        <component :is="NewIngredient_" :input="ingredient_search" @done="close_ingr_modal" @created="add_ingr"></component>
    </dynamic-modal>
    <dynamic-modal ref="modal_unit">
        <component :is="NewUnit_" :input="unit_search" @done="close_unit_modal" @created="set_current_unit"></component>
    </dynamic-modal>
</template>

<script>
import Multiselect from '@vueform/multiselect'
import IngredientListItem from '@/components/IngredientListItem.vue'
import {obj_with_searchable_name} from '@/utils/utils.js'
import DynamicModal from '@/components/DynamicModal.vue'
import NewIngredient from '@/components/NewIngredient.vue'
import NewUnit from '@/components/NewUnit.vue'
import {shallowRef} from 'vue'

export default {
    name: 'ingredient-picker',
    inject: ["store"],
    components : {
        Multiselect,
        IngredientListItem,
        DynamicModal
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
            current_unit_input: null,
            NewIngredient_: shallowRef(NewIngredient),
            NewUnit_: shallowRef(NewUnit)
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
    emits: ['update:picked', 'createIngredient', 'createUnit'],
    methods: {
        pick_ingr(ingr) {
            this.add_ingr(ingr)
            this.$nextTick(() => {
                this.$refs.multiselect.$el.focus()
            })
        },
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
            if (this.current_unit_input !== null)
                this.unit_search = this.current_unit_input.search
        },
        save_current_unit_input(elem) {
            this.current_unit_input = elem
        },
        set_current_unit(unit) {
            if (this.current_unit_input !== undefined)
                this.current_unit_input.select(unit)
        },
        open_ingr_modal() {
            this.$refs.modal_ingr.open()
        },
        close_ingr_modal() {
            this.$refs.modal_ingr.close()
        },
        open_unit_modal() {
            this.$refs.modal_unit.open()
        },
        close_unit_modal() {
            this.$refs.modal_unit.close()
        },
    }
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
