<template>
    <form @submit.prevent="sendIngredient" class="column is-half-desktop is-three-quarters-mobile" autocomplete="off">
    <div class="box">
        <div class="field">
            <label class="label">Nom</label>
            <input v-model="name" class="input" type="text" name="name" id="name" ref="ingrName" required>
        </div>
        <div class="field">
            <label class="label">Unité par défaut</label>
            <button type="button" class="button is-rounded is-primary is-outlined is-small mb-2" @mousedown="save_unit_search" @click="open_modal">Unité manquante ?</button>
            <multiselect @keydown.ctrl.enter.prevent="save_unit_search(), open_modal()" v-model="default_unit" :options="searchableUnits" label="full_name" searchable trackBy="searchable_name" valueProp="id" ref="multiselect"/>
        </div>
        <dynamic-modal ref="modal">
            <component :is="NewUnit_" :input="unit_search" @done="close_modal" @created="set_unit"></component>
        </dynamic-modal>
        <div class="field is-grouped">
          <div class="control">
            <button class="button is-primary" type="submit">Ok</button>
          </div>
          <div class="control">
            <button class="button" type="button" @click="cancel">Annuler</button>
          </div>
        </div>
    </div>
    </form>
</template>

<script>
import Multiselect from '@vueform/multiselect'
import {obj_with_searchable_name} from '@/utils/utils.js'
import DynamicModal from '@/components/DynamicModal.vue'
import {shallowRef} from 'vue'
import NewUnit from '@/components/NewUnit.vue'

export default {
    name: 'new-ingredient',
    inject: ["store"],
    components: {
        Multiselect,
        DynamicModal
    },
    data: function() {
        return {
            name: this.input,
            default_unit: null,
            unit_search: null,
            NewUnit_: shallowRef(NewUnit)
        }
    },
    props: {
        input: null
    },
    computed: {
        searchableUnits() {
            return this.store.state.units.map(unit => obj_with_searchable_name(unit, "full_name"))
        }
    },
    methods: {
        cancel() {
            this.$emit('done')
        },
        sendIngredient() {
            let ingredient = {
                "name": this.name,
                "default_unit_id": this.default_unit
            }
            this.store.addIngredient(ingredient)
                .catch((e) => console.error(e))
                .then((new_ingr) => {
                    this.$emit('created', new_ingr)
                    this.$emit('done')
                })
        },
        save_unit_search() {
            this.unit_search = this.$refs.multiselect.search
        },
        set_unit(unit) {
            this.default_unit = unit.id
        },
        open_modal() {
            this.$refs.modal.open()
        },
        close_modal() {
            this.$refs.modal.close()
        },
    },
    mounted() {
        this.$refs.ingrName.focus()
    },
    emits: ['done', 'created']
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
