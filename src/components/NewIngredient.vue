<template>
    <form @submit.prevent="sendIngredient" class="column is-half-desktop is-three-quarters-mobile">
    <div class="box">
        <div class="field">
            <label class="label">Nom de l'ingrédient</label>
            <input v-model="name" class="input" type="text" name="name" id="name" required>
        </div>
        <div class="field">
            <label class="label">Unité par défaut</label>
            <multiselect v-model="default_unit" :options="store.state.units" label="full_name" searchable trackBy="full_name" valueProp="id"/>
        </div>
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
    components: {
        Multiselect
    },
    data: function() {
        return {
            name: null,
            default_unit: null,
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
    },
    emits: ['done']
}
</script>
