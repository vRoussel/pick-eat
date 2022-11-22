<template>
  <div
    class="modal cursor-pointer"
    :class="{'modal-open': opened}"
    tabindex="-1"
    @click.self="close"
    @keyup.esc.stop="close"
  >
    <div class="modal-box relative overflow-visible cursor-default">
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
            <button
              class="btn rounded-full btn-primary btn-outline btn-sm modal-button"
              @mousedown="save_unit_search"
              @click="open_unit_modal"
              type="button"
            >Unité manquante ?</button>
          </label>
          <multiselect
            ref="multiselect"
            v-model="default_unit"
            :options="searchableUnits"
            label="full_name"
            searchable
            track-by="searchable_name"
            value-prop="id"
            @keydown.ctrl.enter.prevent="save_unit_search(), open_unit_modal()"
          />
        </div>
      </form>
      <div class="modal-action">
        <button
          class="btn btn-primary btn-sm btn-wide mx-auto"
        >Ajouter</button>
      </div>
    </div>
  </div>
  <new-unit-modal
    :input="unit_search"
    @created="set_unit"
    @closed="$refs.ingrName.focus()"
    ref="unit_modal_inner"
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
        input: null
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            name: null,
            default_unit: null,
            unit_search: null,
            opened: false
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
                    this.close()
                })
        },
        save_unit_search() {
            this.unit_search = this.$refs.multiselect.search
        },
        set_unit(unit) {
            this.default_unit = unit.id
        },
        open() {
            this.opened = true
            setTimeout(() => this.$refs.ingrName.focus(), 50)
        },
        close() {
            this.opened = false
            this.name = null
            this.default_unit = null
            this.unit_search = null
            this.$emit('closed')
        },
        open_unit_modal() {
            this.$refs.unit_modal_inner.open()
        }
    }
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
