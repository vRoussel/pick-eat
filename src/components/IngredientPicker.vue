<template>
    <div class="select is-fullwidth block">
        <select v-model="ingr_selected" @change="add_ingr(ingr_selected)">
            <option v-for="ingr in ingr_remaining" :key="ingr.id" :value="ingr">{{ ingr.name }}</option>
        </select>
    </div>
    <div class="columns is-vcentered is-mobile" v-for="ingr in picked.values()" :key="ingr.ingr.id">
        <div class="column is-narrow">
            <button class="delete" type="button" @click="del_ingr(ingr.ingr)"></button>
        </div>
        <span class="column has-text-right"> {{ ingr.ingr.name }}</span>
        <input v-model.number="picked_obj[ingr.ingr.id].quantity" class="input column is-2" type="number">
        <div class="column">
            <div class="select is-fullwidth">
                <select class="column is-6" v-model="picked_obj[ingr.ingr.id].unit_id">
                    <option v-for="unit in unit_choices" :value="unit" :key="unit.id">{{ unit.full_name }}</option>
                </select>
            </div>
        </div>
    </div>
</template>

<script>
//TODO use v-select
export default {
    name: 'ingredient-picker',
    props: {
        ingr_choices: {
            type: Array
        },
        unit_choices: {
            type: Array
        },
        picked: {
            type: Map
        }
    },
    data: function() {
        return {
            ingr_selected: -1,
        }
    },
    computed: {
        picked_obj() {
            return Object.fromEntries(this.picked);
        },
        ingr_remaining() {
            return this.ingr_choices.filter(ingr => !this.picked.has(ingr.id))
        }
    },
    emits: ['update:picked'],
    methods: {
        add_ingr(ingr) {
            this.picked.set(ingr.id, {
                ingr: ingr,
                unit_id: ingr.default_unit,
                quantity: ""
            })
            this.$emit('update:picked', this.picked)
            this.ingr_selected = -1
        },
        del_ingr(ingr) {
            this.picked.delete(ingr.id)
            this.$emit('update:picked', this.picked)
        }
    }
}
</script>
