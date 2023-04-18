<template>
  <div class="flex items-center gap-x-4 w-full">
      <span class="icon text-2xl basis-6 grow-0 text-primary cursor-pointer" @click="$emit('delete')">
        <Icon :icon="icons.close" />
      </span>
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
    inject: ["icons"],
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
