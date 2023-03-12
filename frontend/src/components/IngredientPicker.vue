<template>
  <Multiselect
    ref="multiselect"
    v-model="dummy"
    mode="multiple"
    :options="ingr_remaining"
    label="name"
    searchable
    track-by="name"
    :strict="false"
    object
    value-prop="id"
    placeholder="Ajouter un ingrédient"
    open-direction="top"
    @keydown.ctrl.enter.prevent="save_ingredient_search(), open_ingr_modal()"
    @select="pick_ingr"
  />
  <div class="flex gap-1 my-2">
    <button
      class="btn rounded-full btn-primary btn-outline btn-sm modal-button"
      @mousedown="save_ingredient_search"
      @click="open_ingr_modal"
      type="button"
    >Ingrédient manquant ?</button>
    <button
      class="btn rounded-full btn-primary btn-outline btn-sm modal-button"
      @mousedown="save_unit_search"
      @click="open_unit_modal"
      type="button"
    >Unité manquante ?</button>
  </div>
  <div class="flex flex-col items-center mt-2 gap-y-5">
    <ingredient-list-item
      v-for="ingr in picked.values()"
      :id="ingr.id"
      :key="ingr.id"
      v-model:quantity="ingr.quantity"
      v-model:unit_id="ingr.unit_id"
      @delete="del_ingr(ingr.id)"
      @createUnit="save_unit_search(), open_unit_modal()"
      @unit-input-selected="save_current_unit_input"
    />
  </div>
  <new-ingredient-modal
    :input="ingredient_search"
    @created="add_ingr"
    ref="ingr_modal"
  />
  <new-unit-modal
    :input="unit_search"
    @created="set_current_unit"
    ref="unit_modal"
  />
</template>

<script>
import Multiselect from '@vueform/multiselect'
import IngredientListItem from '@/components/IngredientListItem.vue'
import NewIngredientModal from '@/components/NewIngredientModal.vue'
import NewUnitModal from '@/components/NewUnitModal.vue'

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

export default {
    name: 'IngredientPicker',
    components : {
        Multiselect,
        IngredientListItem,
        NewIngredientModal,
        NewUnitModal
    },
    props: {
        picked: {
            type: Map
        }
    },
    emits: ['update:picked', 'createIngredient', 'createUnit'],
    data: function() {
        return {
            dummy: null,
            ingredient_search: null,
            unit_search: null,
            current_unit_input: null,
        }
    },
    computed: {
        ...mapStores(useFoodStore),
        picked_obj() {
            return Object.fromEntries(this.picked);
        },
        ingr_remaining() {
            return this.foodStore.ingredients
                .filter(ingr => !this.picked.has(ingr.id))
        },
    },
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
            if (this.current_unit_input !== null)
                this.current_unit_input.select(unit)
        },
        open_ingr_modal() {
            this.$refs.ingr_modal.open()
        },
        open_unit_modal() {
            this.$refs.unit_modal.open()
        }
    }
}
</script>
