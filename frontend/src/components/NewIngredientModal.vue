<template>
  <input
    :id="modal_id"
    v-model="modal_opened"
    type="checkbox"
    class="modal-toggle"
  >
  <div
    class="modal"
    tabindex="-1"
    @click.self="modal_opened=false"
    @keyup.esc.stop="modal_opened=false"
  >
    <div class="modal-box relative overflow-visible">
      <form
        autocomplete="off"
        @submit.prevent="sendIngredient"
      >
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Nom</span>
          </label>
          <input
            id="name"
            ref="ingrName"
            v-model="name"
            class="input input-bordered w-full"
            type="text"
            name="name"
            required
          >
        </div>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Unité par défaut</span>
            <label
              for="modal_unit2"
              class="btn rounded-full btn-primary btn-outline btn-sm modal-button"
              @mousedown="save_unit_search"
            >Unité manquante ?</label>
          </label>
          <multiselect
            ref="multiselect"
            v-model="default_unit"
            :options="searchableUnits"
            label="full_name"
            searchable
            track-by="searchable_name"
            value-prop="id"
            @keydown.ctrl.enter.prevent="save_unit_search(), open_modal()"
          />
        </div>
      </form>
      <div class="modal-action">
        <label
          :for="modal_id"
          class="btn btn-primary btn-sm btn-wide mx-auto"
          @click="sendIngredient"
        >Ajouter</label>
      </div>
    </div>
  </div>
  <new-unit-modal
    modal_id="modal_unit2"
    :input="unit_search"
    @created="set_unit"
    @closed="$refs.ingrName.focus()"
  />
</template>

<script>
import Multiselect from '@vueform/multiselect'
import {obj_with_searchable_name} from '@/utils/utils.js'
import NewUnitModal from '@/components/NewUnitModal.vue'

import { mapStores } from 'pinia'
import { useApiStore } from '@/store/api.js'

export default {
    name: 'NewIngredientModal',
    components: {
        Multiselect,
        NewUnitModal
    },
    props: {
        modal_id: {
            required: true
        },
        input: null
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            name: null,
            default_unit: null,
            unit_search: null,
            modal_opened: false
        }
    },
    computed: {
        ...mapStores(useApiStore),
        searchableUnits() {
            return this.apiStore.units.map(unit => obj_with_searchable_name(unit, "full_name"))
        }
    },
    watch: {
        input: function() {
            this.name = this.input;
        },
        modal_opened(val) {
            if (val) {
                this.$refs.ingrName.focus()
            } else {
                this.name = null
                this.default_unit = null
                this.unit_search = null
                this.$emit('closed')
            }
        }
    },
    methods: {
        sendIngredient() {
            let ingredient = {
                "name": this.name,
                "default_unit_id": this.default_unit
            }
            this.apiStore.sendNewIngredient(ingredient)
                .catch((e) => console.error(e))
                .then((new_ingr) => {
                    this.$emit('created', new_ingr)
                    this.modal_opened = false
                })
        },
        save_unit_search() {
            this.unit_search = this.$refs.multiselect.search
        },
        set_unit(unit) {
            this.default_unit = unit.id
        },
    }
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
