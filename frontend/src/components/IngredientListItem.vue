<script setup>
import { inject, ref, computed } from 'vue'

import Multiselect from '@vueform/multiselect'
import NumberInput from '@/components/NumberInput.vue'
import { useFoodStore } from '@/store/food.js'

const foodStore = useFoodStore()

const icons = inject('icons')

const model_unit_id = defineModel('unit_id', {
    type: Number
})

const model_quantity = defineModel('quantity', {
    type: Number
})

const props = defineProps({
    id: Number,
});

const emit = defineEmits(['delete', 'createUnit', 'unit-input-selected'])

const ingredient_unit = computed({
    get() {
        return model_unit_id.value
    },
    set(v) {
        model_unit_id.value = v
    },
})

const ingredient_quantity = computed({
    get() {
        return model_quantity.value
    },
    set(v) {
        model_quantity.value = (v == '' ? null : v)
    }
})

const ingredient_name = computed(() => {
    return foodStore.getIngredientById(props.id).name
})

function create_unit() {
    emit('createUnit')
}

const multiselect_el = ref(null)
function notify_input_selected() {
    emit('unit-input-selected', multiselect_el)
}
</script>

<template>
    <div class="flex items-center gap-x-4 w-full">
        <span class="icon text-2xl basis-6 grow-0 text-primary cursor-pointer" @click="emit('delete')">
            <Icon :icon="icons.close" />
        </span>
        <div class="flex flex-wrap items-center gap-2 grow">
            <span class="sm:text-end basis-full sm:basis-3/12 grow shrink" tabindex="-1">
                {{ ingredient_name }}</span>
            <number-input v-model.number="ingredient_quantity" :min="0" placeholder="Quantité"
                class="basis-5/12 sm:basis-3/12" />
            <multiselect ref="multiselect_el" v-model="ingredient_unit" class="basis-5/12 sm:basis-1/3 flex-grow"
                :options="foodStore.units" :strict="false" label="full_name" searchable track-by="full_name" value-prop="id"
                placeholder="Unité" @keydown.ctrl.enter.prevent="create_unit()" @open="notify_input_selected" />
        </div>
    </div>
</template>
