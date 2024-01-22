<script setup>

import { ref, nextTick, computed } from 'vue'

import Multiselect from '@vueform/multiselect'
import IngredientListItem from '@/components/IngredientListItem.vue'
import NewIngredientModal from '@/components/NewIngredientModal.vue'
import NewUnitModal from '@/components/NewUnitModal.vue'
import { useFoodStore } from '@/store/food.js'

const foodStore = useFoodStore()

const model = defineModel('picked', {
    type: Map,
    required: true
})

const dummy = ref(null)
const ingredient_search = ref(null)
const unit_search = ref(null)

const multiselect_el = ref(null)
const ingredient_modal_el = ref(null)
const unit_modal_el = ref(null)
const current_unit_input_el = ref(null)

const ingr_remaining = computed(() => {
    return foodStore.ingredients.filter((ingr) => !model.value.has(ingr.id))
})

const _picked = computed(() => {
    return new Map(model.value)
})

function add_ingr(ingr) {
    _picked.value.set(ingr.id, {
        id: ingr.id,
        unit_id: ingr.default_unit ? ingr.default_unit.id : null,
        quantity: null,
    })
    model.value = _picked.value
    multiselect_el.value.clear()
    nextTick(() => {
        multiselect_el.value.focus()
    })
}

function del_ingr(id) {
    _picked.value.delete(id)
    model.value = _picked.value
}

function save_ingredient_search() {
    ingredient_search.value = multiselect_el.value.search
}

function save_unit_search() {
    if (current_unit_input_el.value !== null)
        unit_search.value = current_unit_input_el.value.search
}

function save_current_unit_input(elem) {
    current_unit_input_el.value = elem
}

function set_current_unit(unit) {
    if (current_unit_input_el.value !== null)
        current_unit_input_el.value.select(unit)
}

function open_ingr_modal() {
    ingredient_modal_el.value.open()
}

function open_unit_modal() {
    unit_modal_el.value.open()
}
</script>

<template>
    <div v-bind="$attrs" class="rounded-md">
        <Multiselect ref="multiselect_el" v-model="dummy" mode="multiple" :options="ingr_remaining" label="name" searchable
            track-by="name" :strict="false" object value-prop="id" placeholder="Ajouter un ingrédient" open-direction="top"
            @keydown.ctrl.enter.prevent="save_ingredient_search(), open_ingr_modal()" @select="add_ingr" />
    </div>
    <div class="flex gap-1 my-2">
        <button class="btn rounded-full btn-primary btn-outline btn-sm modal-button" type="button" tabindex="-1"
            @mousedown="save_ingredient_search" @click="open_ingr_modal">
            Ingrédient manquant ?
        </button>
        <button class="btn rounded-full btn-primary btn-outline btn-sm modal-button" type="button" tabindex="-1"
            @mousedown="save_unit_search" @click="open_unit_modal">
            Unité manquante ?
        </button>
    </div>
    <div class="flex flex-col items-center mt-2 gap-y-5">
        <ingredient-list-item v-for="ingr in model.values()" :id="ingr.id" :key="ingr.id" v-model:quantity="ingr.quantity"
            v-model:unit_id="ingr.unit_id" @delete="del_ingr(ingr.id)" @createUnit="save_unit_search(), open_unit_modal()"
            @unit-input-selected="save_current_unit_input" />
    </div>
    <new-ingredient-modal ref="ingredient_modal_el" :input="ingredient_search" @created="add_ingr" />
    <new-unit-modal ref="unit_modal_el" :input="unit_search" @created="set_current_unit" />
</template>
