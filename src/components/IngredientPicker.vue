<template>
    <div class="select is-fullwidth block">
        <select v-model="ingr_selected" @change="add_ingr(ingr_selected)">
            <option v-for="ingr in ingr_remaining" :key="ingr.id" :value="ingr">{{ ingr.name }}</option>
        </select>
    </div>
    <div class="columns is-vcentered is-mobile" v-for="ingr in picked.values()" :key="ingr.id">
        <div class="column is-narrow">
            <button class="delete" type="button" @click="del_ingr(ingr.id)"></button>
        </div>
        <span class="column has-text-right"> {{ ingr_by_id.get(ingr.id).name }}</span>
        <input v-model.number="picked_obj[ingr.id].quantity" class="input column is-2" type="number">
        <div class="column">
            <div class="select is-fullwidth">
                <select class="column is-6" v-model="picked_obj[ingr.id].unit_id">
                    <option v-for="unit in store.units" :value="unit.id" :key="unit.id">{{ unit.full_name }}</option>
                </select>
            </div>
        </div>
    </div>
</template>

<script>
import store from '@/store/store.js'
//TODO use v-select
export default {
    name: 'ingredient-picker',
    props: {
        picked: {
            type: Map
        }
    },
    data: function() {
        return {
            ingr_selected: -1,
            store: store
        }
    },
    computed: {
        picked_obj() {
            return Object.fromEntries(this.picked);
        },
        ingr_remaining() {
            return store.ingredients.filter(ingr => !this.picked.has(ingr.id))
        },
        ingr_by_id() {
            return new Map(store.ingredients.map(ingr => [ingr.id, ingr]))
        }
    },
    emits: ['update:picked'],
    methods: {
        add_ingr(ingr) {
            this.picked.set(ingr.id, {
                id: ingr.id,
                unit_id: ingr.default_unit.id,
                quantity: ""
            })
            this.$emit('update:picked', this.picked)
            this.ingr_selected = -1
        },
        del_ingr(id) {
            this.picked.delete(id)
            this.$emit('update:picked', this.picked)
        }
    }
}
</script>
