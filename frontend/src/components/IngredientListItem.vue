<template>
  <div class="flex items-center gap-x-4 w-full">
    <button
      class="btn btn-circle btn-xs btn-outline basis-6 grow-0"
      tabindex="-1"
      type="button"
      @click="$emit('delete')"
    >
      <svg viewBox="0 0 512 512"><path d="M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM175 208.1L222.1 255.1L175 303C165.7 312.4 165.7 327.6 175 336.1C184.4 346.3 199.6 346.3 208.1 336.1L255.1 289.9L303 336.1C312.4 346.3 327.6 346.3 336.1 336.1C346.3 327.6 346.3 312.4 336.1 303L289.9 255.1L336.1 208.1C346.3 199.6 346.3 184.4 336.1 175C327.6 165.7 312.4 165.7 303 175L255.1 222.1L208.1 175C199.6 165.7 184.4 165.7 175 175C165.7 184.4 165.7 199.6 175 208.1V208.1z" /></svg>
    </button>
    <div class="flex flex-wrap items-center gap-2 grow">
      <span
        class="sm:text-end basis-full sm:basis-3/12 grow shrink"
        tabindex="-1"
      > {{ ingredient_name }}</span>
      <number-input
        v-model.number="ingredient_quantity"
        :min="0"
        placeholder="Quantité"
        class="basis-5/12 sm:basis-3/12"
      />
      <multiselect
        ref="multiselect"
        v-model="ingredient_unit"
        class="basis-5/12 sm:basis-1/3 flex-grow"
        :options="foodStore.units"
        :strict="false"
        label="full_name"
        searchable
        track-by="full_name"
        value-prop="id"
        placeholder="Unité"
        @keydown.ctrl.enter.prevent="create_unit()"
        @open="notify_input_selected"
      />
    </div>
  </div>
</template>

<script>
import Multiselect from '@vueform/multiselect'
import NumberInput from '@/components/NumberInput.vue'

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'

export default {
    name: 'IngredientListItem',
    components : {
        Multiselect,
        NumberInput
    },
    props: {
        id: Number,
        quantity: Number,
        unit_id: Number
    },
    emits: ['update:quantity', 'update:unit_id', 'delete', 'createUnit', 'unit-input-selected'],
    computed: {
        ...mapStores(useFoodStore),
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
            return this.foodStore.getIngredientById(this.id).name
        },
    },
    methods: {
        create_unit() {
            this.$emit('createUnit')
        },
        notify_input_selected() {
            this.$emit('unit-input-selected', this.$refs.multiselect)
        }
    },
}
</script>
