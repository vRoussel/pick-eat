<template>
    <Multiselect :options="ingr_remaining" label="name" searchable @select="add_ingr" trackBy="name" object valueProp="id" v-model="dummy" ref="multiselect"/>
    <div class="columns is-vcentered is-mobile" v-for="ingr in picked.values()" :key="ingr.id">
        <ingredient-list-item @delete="del_ingr(ingr.id)" v-model:quantity="ingr.quantity" :id="ingr.id" v-model:unit_id="ingr.unit_id"></ingredient-list-item>
    </div>
</template>

<script>
import store from '@/store/store.js'
import Multiselect from '@vueform/multiselect'
import IngredientListItem from '@/components/IngredientListItem.vue'

export default {
    name: 'ingredient-picker',
    components : {
        Multiselect,
        IngredientListItem
    },
    props: {
        picked: {
            type: Map
        }
    },
    data: function() {
        return {
            store: store,
            dummy: null,
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
                unit_id: ingr.default_unit ? ingr.default_unit.id : null,
                quantity: null,
            })
            this.$emit('update:picked', this.picked)
            this.$refs.multiselect.clear()
        },
        del_ingr(id) {
            this.picked.delete(id)
            this.$emit('update:picked', this.picked)
        }
    }
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
