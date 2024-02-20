<script setup>
import { ref, computed, inject } from 'vue'

import GroceryListItem from '@/components/GroceryListItem.vue'
import { useCartStore } from '@/store/cart.js'
import { useFoodStore } from '@/store/food.js'
import { useNotifStore } from '@/store/notif.js'

import { qty_scaled } from '@/utils/utils.js'


let cartStore = useCartStore()
let foodStore = useFoodStore()
const notifStore = useNotifStore()

const icons = inject('icons')

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
                let qty = qty_scaled(ingr.quantity, ratio, 0.25)
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

async function copyIngredients() {
    let ret = ""
    for (const [ingr_id, qu] of list.value) {
        let ingr_name = foodStore.getIngredientById(ingr_id).name
        let quantities = new Array()
        for (let [unit_id, qtys] of qu.q) {
            let total_qty = qtys.reduce((s, q) => s + q.qty, 0)
            if (unit_id) {
                let unit_name = foodStore.getUnitById(unit_id).short_name
                quantities.push(`${total_qty} ${unit_name}`)
            } else {
                quantities.push(`${total_qty}`)
            }
        }
        if (qu.u.length > 0) {
            let word = qu.u.length > 1 ? 'recettes' : 'recette'
            quantities.push(`un peu (${qu.u.length} ${word})`)
        }

        ret += `${ingr_name} x ${quantities.join(' + ')}`
        ret += '\n'
    }
    try {
        await navigator.clipboard.writeText(ret)
        notifStore.show_info("Liste de course copiée dans le presse-papier")
    } catch (e) {
        notifStore.show_error("Impossible de copier la liste de course dans le presse-papier")
    }
}

defineExpose({ open });
</script>

<template>
    <div class="modal cursor-pointer" :class="{ 'modal-open': opened }" tabindex="-1" @click.self="close"
        @keyup.esc.stop="close">
        <div class="modal-box relative overflow-y-scroll max-w-xl cursor-default p-4 sm:p-6">
            <div v-if="cartStore.recipeCount > 0" class="space-y-4">
                <div class="tabs tabs-boxed mb-4">
                    <a class="tab grow" :class="{ 'tab-active': tab == 0 }" @click="tab = 0">Ingrédients</a>
                    <a class="tab grow" :class="{ 'tab-active': tab == 1 }" @click="tab = 1">Recettes</a>
                </div>
                <table v-if="tab == 0" class="table leading-none table-zebra mx-auto pt-4">
                    <tbody>
                        <tr v-for="[ingr_id, qu] in list" :key="ingr_id">
                            <td class="text-right">
                                {{ foodStore.getIngredientById(ingr_id).name }}
                            </td>
                            <td class="w-4">x</td>
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
                                    <span class="tooltip italic" :data-tip="qu.u.map((x) => x.r_name).join(', ')">un
                                        peu</span>
                                </span>
                            </td>
                        </tr>
                    </tbody>
                </table>
                <div class="flex justify-center">
                    <button v-if="tab == 0" class="btn btn-sm btn-primary btn-outline btn-wide" @click="copyIngredients">
                        Copier la liste des ingredients
                        <Icon class="text-primary cursor-pointer" :icon="icons.copy" @click="" />
                    </button>
                </div>
                <div v-if="tab == 1" class="flex flex-col items-center space-y-4">
                    <grocery-list-item v-for="[r_id, val] in cartStore.content" :key="r_id" :recipe_id="r_id"
                        :recipe_name="val.recipe.name" :shares="val.shares" :shares_unit="val.recipe.shares_unit"
                        @recipe-opened="close" />
                </div>
            </div>
            <p v-else>
                Ajoutez au moins une recette dans votre panier pour afficher la liste de courses.
            </p>
        </div>
    </div>
</template>
