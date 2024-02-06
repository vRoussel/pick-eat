<script setup>
import { ref, onMounted, nextTick } from 'vue'

import NumberInput from '@/components/NumberInput.vue'
import { useCartStore } from '@/store/cart.js'

import { isOverflown } from '@/utils/utils.js'

const cartStore = useCartStore()

const props = defineProps({
    recipe_id: {
        type: Number,
        required: true
    },
    recipe_name: {
        type: String,
        required: true
    },
    shares: {
        type: Number,
        required: true
    },
    shares_unit: {
        type: String,
        required: true
    }
});

const overflown_recipe_name = ref(false);
const overflown_shares_unit = ref(false);

const recipe_name_input_el = ref(null);
const shares_unit_el = ref(null);
onMounted(async () => {
    await nextTick()
    overflown_recipe_name.value = isOverflown(recipe_name_input_el.value.$el)
    overflown_shares_unit.value = isOverflown(shares_unit_el.value)

})
</script>

<template>
    <div class="flex items-center gap-x-2 sm:gap-x-4 w-full justify-stretch">
        <router-link ref="recipe_name_input_el" v-tooltip="overflown_recipe_name ? recipe_name : null"
            class="text-end link-hover truncate basis-4/12 grow" :to="'/recipe/' + recipe_id" tabindex="-1">
            {{ recipe_name }}
        </router-link>
        <span> x </span>
        <number-input class="basis-10" :model-value="shares" :min="0"
            @update:modelValue="(val) => cartStore.updateRecipeShares(recipe_id, val)" />
        <span ref="shares_unit_el" v-tooltip="overflown_shares_unit ? shares_unit : null" class="truncate basis-2/12"> {{
            shares_unit }}</span>
        <button class="btn btn-circle btn-xs btn-outline" tabindex="-1" type="button"
            @click="cartStore.removeRecipe(recipe_id)">
            <svg viewBox="0 0 512 512">
                <path
                    d="M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM175 208.1L222.1 255.1L175 303C165.7 312.4 165.7 327.6 175 336.1C184.4 346.3 199.6 346.3 208.1 336.1L255.1 289.9L303 336.1C312.4 346.3 327.6 346.3 336.1 336.1C346.3 327.6 346.3 312.4 336.1 303L289.9 255.1L336.1 208.1C346.3 199.6 346.3 184.4 336.1 175C327.6 165.7 312.4 165.7 303 175L255.1 222.1L208.1 175C199.6 165.7 184.4 165.7 175 175C165.7 184.4 165.7 199.6 175 208.1V208.1z" />
            </svg>
        </button>
    </div>
</template>

