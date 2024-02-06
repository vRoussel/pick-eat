<script setup>
import { ref, computed } from 'vue'

import GroceryListItem from '@/components/GroceryListItem.vue'
import { useCartStore } from '@/store/cart.js'
import { useFoodStore } from '@/store/food.js'


let cartStore = useCartStore()
let foodStore = useFoodStore()

const opened = ref(false);
const tab = ref(0);

/*
{
    ingr_id => {
        q => {
            unit_id => [
                {r_id: X, r_name: Y, qty: Z},
                ...
            ],
            ...
        },
        u => [
            {r_id: X, r_name: Y},
            ...
        ],
        }
    },
    ...
}
*/
const list = computed(() => {
    let list = new Map()
    for (const val of cartStore.content.values()) {
        let recipe_id = val.recipe.id
        let recipe_name = val.recipe.name
        let ingredients = val.recipe.q_ingredients
        let ratio = val.shares / val.recipe.n_shares
        for (const ingr of ingredients.values()) {
            if (!list.has(ingr.id)) list.set(ingr.id, { q: new Map(), u: new Array() })

            if (ingr.quantity) {
                let q = list.get(ingr.id).q
                let unit_id = ingr.unit ? ingr.unit.id : null
                if (!q.has(unit_id)) q.set(unit_id, new Array())
                let unit_qtys = q.get(unit_id)
                let qty = Math.round((ingr.quantity * ratio + Number.EPSILON) * 100) / 100
                unit_qtys.push({ r_id: recipe_id, r_name: recipe_name, qty: qty })
            } else {
                let u = list.get(ingr.id).u
                u.push({ r_id: recipe_id, r_name: recipe_name })
            }
        }
    }
    return list
})

function open() {
    opened.value = true
    tab.value = 0
}

function close() {
    opened.value = false
}

defineExpose({ open });
</script>

<template>
    <div class="modal cursor-pointer" :class="{ 'modal-open': opened }" tabindex="-1" @click.self="close"
        @keyup.esc.stop="close">
        <div class="modal-box relative overflow-y-scroll max-w-md cursor-default">
            <div v-if="cartStore.recipeCount > 0" class="space-y-4">
                <div class="tabs tabs-boxed">
                    <a class="tab grow" :class="{ 'tab-active': tab == 0 }" @click="tab = 0">Ingrédients</a>
                    <a class="tab grow" :class="{ 'tab-active': tab == 1 }" @click="tab = 1">Recettes</a>
                </div>
                <table v-if="tab == 0" class="table leading-none table-zebra mx-auto pt-4">
                    <tbody>
                        <tr v-for="[ingr_id, qu] in list" :key="ingr_id">
                            <td class="text-right">
                                {{ foodStore.getIngredientById(ingr_id).name }}
                            </td>
                            <td>x</td>
                            <td>
                                <span v-for="([unit_id, qtys], idx) in qu.q" :key="unit_id">
                                    <span v-if="idx > 0"> + </span>
                                    {{ qtys.reduce((s, q) => s + q.qty, 0) }}
                                    {{
                                        unit_id == null
                                        ? ''
                                        : foodStore.getUnitById(unit_id).short_name
                                    }}
                                </span>
                                <span v-if="qu.u.length > 0">
                                    <span v-if="qu.q.size > 0"> + </span>
                                    <span v-tooltip="qu.u.map((x) => x.r_name).join(', ')">...</span>
                                </span>
                            </td>
                        </tr>
                    </tbody>
                </table>
                <div v-if="tab == 1" class="flex flex-col items-center space-y-4">
                    <div v-for="[r_id, val] in cartStore.content" :key="r_id" class="flex items-center gap-x-4 w-full">
                        <grocery-list-item :recipe_id="r_id" :recipe_name="val.recipe.name" :shares="val.shares"
                            :shares_unit="val.recipe.shares_unit" />
                    </div>
                </div>
            </div>
            <p v-else>
                Ajoutez au moins une recette dans votre panier pour afficher la liste de courses.
            </p>
        </div>
    </div>
</template>
