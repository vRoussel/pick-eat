<template>
    <form @submit.prevent="sendIngredient" class="column is-half-desktop is-three-quarters-mobile" autocomplete="off">
    <div class="box">
        <div class="field">
            <label class="label">Nom</label>
            <input v-model="name" class="input" type="text" name="name" id="name" ref="ingrName" required>
        </div>
        <div class="field">
            <label class="label">Unité par défaut</label>
            <button type="button" class="button is-rounded is-info is-outlined is-small mb-2" @click="openNewUnitForm">Unité manquante ?</button>
            <multiselect v-model="default_unit" :options="store.state.units" label="full_name" searchable trackBy="full_name" valueProp="id"/>
        </div>
        <dynamic-modal v-model:currentComponent="currentModalContent"></dynamic-modal>
        <div class="field is-grouped">
          <div class="control">
            <button class="button is-success" type="submit">Ok</button>
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

export default {
    name: 'new-ingredient',
    inject: ["store"],
    beforeCreate: function () {
        // Necessary because of circular dependency between DyanmicModal and NewIngredient 
        this.$options.components.DynamicModal = require('@/components/DynamicModal.vue').default
    },
    components: {
        Multiselect,
    },
    data: function() {
        return {
            name: null,
            default_unit: null,
            currentModalContent: null,
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
                .then(() => this.store.getIngredients())
                .catch((e) => console.error(e))
            this.$emit('done')
        },
        openNewUnitForm() {
            this.currentModalContent = "NewUnit"
        }
    },
    mounted() {
        this.$refs.ingrName.focus()
    },
    emits: ['done']
}
</script>
