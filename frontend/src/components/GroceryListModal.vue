<template>
    <input v-model="modal_opened" type="checkbox" :id="modal_id" class="modal-toggle" />
    <div class="modal" @click.self="this.modal_opened=false" @keyup.esc.stop="this.modal_opened=false" tabindex="-1">
        <div class="modal-box relative overflow-y-scroll max-w-md space-y-4">
            <div class="tabs tabs-boxed">
                <a class="tab grow" :class='{"tab-active": this.tab == 0}' @click="this.tab = 0">Ingrédients</a>
                <a class="tab grow" :class='{"tab-active": this.tab == 1}' @click="this.tab = 1">Recettes</a>
            </div>
            <table v-if="this.tab == 0" class="table leading-none table-zebra mx-auto pt-4">
            <tbody>
                    <tr v-for="[ingr_id, qu] in list" :key="ingr_id">
                    <td class="text-right">
                        {{ this.store.getIngredientById(ingr_id) }}
                    </td>
                    <td> x </td>
                    <td>
                        <span v-for="([unit_id, qtys], idx) in qu.q" :key="unit_id">
                            <span v-if="idx > 0"> + </span>
                            {{ qtys.reduce((s,q) => s + q.qty, 0) }} {{ this.store.getUnitById(unit_id) }}
                        </span>
                        <span v-if="qu.u.length > 0">
                            <span v-if="qu.q.size > 0"> + </span>
                            <span v-tooltip="qu.u.map(x => x.r_name).join(', ')">...</span>
                        </span>
                    </td>
                    </tr>
                </tbody>
            </table>
            <div v-if="this.tab == 1" class="flex flex-col items-center space-y-4">
                <div v-for="[r_id, val] in this.cart.content" :key="r_id" class="flex items-center gap-x-4 w-full">
                    <span class="text-end truncate grow">{{ val.recipe.name }}</span>
                    <span> x </span>
                    <number-input class="basis-10 grow-0" :modelValue="val.shares" @update:modelValue="(val) => this.cart.updateShares(r_id, val)" :min="0"/>
                    <span> parts</span>
                    <button class="btn btn-circle btn-xs btn-outline grow-0" tabindex="-1" type="button" @click="this.cart.removeRecipeWithId(r_id)">
                      <svg viewBox="0 0 512 512"><path d="M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM175 208.1L222.1 255.1L175 303C165.7 312.4 165.7 327.6 175 336.1C184.4 346.3 199.6 346.3 208.1 336.1L255.1 289.9L303 336.1C312.4 346.3 327.6 346.3 336.1 336.1C346.3 327.6 346.3 312.4 336.1 303L289.9 255.1L336.1 208.1C346.3 199.6 346.3 184.4 336.1 175C327.6 165.7 312.4 165.7 303 175L255.1 222.1L208.1 175C199.6 165.7 184.4 165.7 175 175C165.7 184.4 165.7 199.6 175 208.1V208.1z"/></svg>
                    </button>
                </div>
            </div>
                    <!--
            <table v-if="this.tab == 1" class="table table-zebra mx-auto p-4">
                <tbody>
                    <tr v-for="[r_id, val] in this.store.state.cart" :key="r_id">
                    <td class="text-right">
                        <p class="truncate max-w-[20ch]">{{ val.recipe.name }}</p>
                    </td>
                    <td>
                        <number-input :modelValue="val.shares" @update:modelValue="(val) => this.store.updateShares(r_id, val)"  :min="0"/>
                    </td>
                    <td>
                        <button class="btn btn-circle btn-xs btn-outline basis-6 grow-0" tabindex="-1" type="button" @click="this.store.removeIdFromCart(r_id)">
                          <svg viewBox="0 0 512 512"><path d="M0 256C0 114.6 114.6 0 256 0C397.4 0 512 114.6 512 256C512 397.4 397.4 512 256 512C114.6 512 0 397.4 0 256zM175 208.1L222.1 255.1L175 303C165.7 312.4 165.7 327.6 175 336.1C184.4 346.3 199.6 346.3 208.1 336.1L255.1 289.9L303 336.1C312.4 346.3 327.6 346.3 336.1 336.1C346.3 327.6 346.3 312.4 336.1 303L289.9 255.1L336.1 208.1C346.3 199.6 346.3 184.4 336.1 175C327.6 165.7 312.4 165.7 303 175L255.1 222.1L208.1 175C199.6 165.7 184.4 165.7 175 175C165.7 184.4 165.7 199.6 175 208.1V208.1z"/></svg>
                        </button>
                    </td>
                    <td>
                        <span v-for="([unit_id, qtys], idx) in qu.q" :key="unit_id">
                            <span v-if="idx > 0"> + </span>
                            {{ qtys.reduce((s,q) => s + q.qty, 0) }} {{ this.store.getUnitById(unit_id) }}
                        </span>
                        <span v-if="qu.u.length > 0">
                            <span v-if="qu.q.size > 0"> + </span>
                            <span v-tooltip="qu.u.map(x => x.r_name).join(', ')">...</span>
                        </span>
                    </td>
                    </tr>
                </tbody>
            </table>
                    -->

        </div>
    </div>
</template>

<script>
import NumberInput from '@/components/NumberInput.vue'
export default {
    name: 'grocery-list-modal',
    inject: ["store", "cart"],
    components : {
        NumberInput
    },
    data: function() {
        return {
            modal_opened: false,
            tab: 0
        }
    },
    props: {
        modal_id: {
            required: true
        }
    },
    computed: {
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
        list() {
            let list = new Map()
            for (const val of this.cart.content.values()) {
                let recipe_id = val.recipe.id
                let recipe_name = val.recipe.name
                let ingredients = val.recipe.q_ingredients
                let ratio = val.shares / val.recipe.n_shares
                for (const ingr of ingredients.values()) {
                    if (!list.has(ingr.id))
                        list.set( ingr.id, {'q': new Map(), 'u': new Array()} )

                    if (ingr.quantity) {
                        let q = list.get(ingr.id).q
                        let unit_id = ingr.unit ? ingr.unit.id : null
                        if (!q.has(unit_id))
                            q.set(unit_id, new Array())
                        let unit_qtys = q.get(unit_id)
                        let qty = Math.round((ingr.quantity * ratio + Number.EPSILON) * 100) / 100;
                        unit_qtys.push({'r_id': recipe_id, 'r_name': recipe_name, 'qty': qty})
                    } else {
                        let u = list.get(ingr.id).u
                        u.push({'r_id': recipe_id, 'r_name': recipe_name})
                    }
                }
            }
            console.log(list)
            return list
        }
    },
    watch: {
        modal_opened(val) {
            if (val) {
                this.tab = 0
            }
        }
    },
}
</script>
