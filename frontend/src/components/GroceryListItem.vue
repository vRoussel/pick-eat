<template>
  <router-link
    ref="recipe_name"
    v-tooltip="overflown ? recipe_name : null"
    class="text-end truncate grow link-hover"
    :to="'/recipe/' + recipe_id"
  >
    {{ recipe_name }}
  </router-link>
  <span> x </span>
  <number-input
    class="basis-10 grow-0"
    :model-value="shares"
    :min="0"
    @update:modelValue="(val) => cartStore.updateRecipeShares(recipe_id, val)"
  />
  <span> parts</span>
  <button
    class="btn btn-circle btn-xs btn-outline grow-0"
    tabindex="-1"
    type="button"
    @click="cartStore.removeRecipe(recipe_id)"
  >
    <svg viewBox="0 0 512 512"><path d="M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM175 208.1L222.1 255.1L175 303C165.7 312.4 165.7 327.6 175 336.1C184.4 346.3 199.6 346.3 208.1 336.1L255.1 289.9L303 336.1C312.4 346.3 327.6 346.3 336.1 336.1C346.3 327.6 346.3 312.4 336.1 303L289.9 255.1L336.1 208.1C346.3 199.6 346.3 184.4 336.1 175C327.6 165.7 312.4 165.7 303 175L255.1 222.1L208.1 175C199.6 165.7 184.4 165.7 175 175C165.7 184.4 165.7 199.6 175 208.1V208.1z" /></svg>
  </button>
</template>

<script>
import NumberInput from '@/components/NumberInput.vue'
import {isOverflown} from '@/utils/utils.js'

import { mapStores } from 'pinia'
import { useCartStore } from '@/store/cart.js'

export default {
    name: 'GroceryListItem',
    components : {
        NumberInput
    },
    props: {
        recipe_id: {
            required: true
        },
        recipe_name: {
            required: true
        },
        shares: {
            required: true
        }
    },
    data: function() {
        return {
            overflown: false,
        }
    },
    computed: {
        ...mapStores(useCartStore)
    },
    mounted() {
        //https://jefrydco.id/en/blog/safe-access-vue-refs-undefined
        const interval = setInterval(() => {
            if (this.$refs.recipe_name) {
                this.overflown = isOverflown(this.$refs.recipe_name.$el)
                clearInterval(interval)
            }
        }, 100)
    },
}
</script>
