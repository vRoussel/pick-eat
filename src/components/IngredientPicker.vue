<template>
    <multiselect :options="ingr_remaining" label="name" track-by="id" placeholder="Miam" @select="add_ingr" :clear-on-select="true" :closeOnSelect="false"></multiselect>
    <div class="columns is-vcentered is-mobile" v-for="ingr in picked.values()" :key="ingr.id">
        <div class="column is-narrow">
            <button class="delete" type="button" @click="del_ingr(ingr.id)"></button>
        </div>
        <span class="column has-text-right"> {{ ingr_by_id.get(ingr.id).name }}</span>
        <input v-model.number="picked_obj[ingr.id].quantity" class="input column is-2" type="number">
        <div class="column">
            <multiselect v-model="picked_obj[ingr.id].unit_id" :options="store.units" label="full_name" track-by="id" placeholder="Unité"></multiselect>
        </div>
    </div>
</template>

<script>
import store from '@/store/store.js'
import Multiselect from '@suadelabs/vue3-multiselect'

export default {
    name: 'ingredient-picker',
    components : {
        Multiselect
    },
    props: {
        picked: {
            type: Map
        }
    },
    data: function() {
        return {
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
            console.log(ingr)
            this.picked.set(ingr.id, {
                id: ingr.id,
                unit_id: ingr.default_unit ? ingr.default_unit.id : null,
                quantity: ""
            })
            this.$emit('update:picked', this.picked)
        },
        del_ingr(id) {
            this.picked.delete(id)
            this.$emit('update:picked', this.picked)
        }
    }
}
</script>

<style src="@suadelabs/vue3-multiselect/dist/vue3-multiselect.css"></style>
