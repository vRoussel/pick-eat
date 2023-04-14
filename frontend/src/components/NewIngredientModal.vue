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
            :class="errors.name && '!input-error'"
            @blur="validate('name')"
          >
          <label class="label" v-if="this.errors.name">
              <span class="label-text-alt text-error">{{ errors.name }}</span>
          </label>
        </div>
        <div class="form-control w-full">
          <label for="" class="label">
            <span class="label-text">Unité par défaut</span>
            <button
              class="btn rounded-full btn-primary btn-outline btn-sm modal-button"
              @mousedown="save_unit_search"
              @click="open_unit_modal"
              type="button"
              tabindex=-1
            >Unité manquante ?</button>
          </label>
          <multiselect
            ref="multiselect"
            v-model="default_unit"
            :options="foodStore.units"
            :strict="false"
            label="full_name"
            searchable
            track-by="full_name"
            value-prop="id"
            @keydown.ctrl.enter.prevent="save_unit_search(), open_unit_modal()"
          />
        </div>
        <div class="modal-action">
          <button
            class="btn btn-primary btn-sm btn-wide mx-auto"
          >Ajouter</button>
        </div>
      </form>
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
import NewUnitModal from '@/components/NewUnitModal.vue'

import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'
import { useNotifStore } from '@/store/notif.js'
import {handle_form_api_errors, handle_form_local_errors} from '@/utils/utils.js'
import { object, string } from "yup";

const validator = object().shape({
    name: string()
            .required("Le nom de l'ingrédient est obligatoire"),
})

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
            opened: false,
            errors: {
                name: null
            }
        }
    },
    computed: {
        ...mapStores(useFoodStore, useNotifStore),
    },
    watch: {
        input: function() {
            this.name = this.input;
        }
    },
    methods: {
        sendIngredient() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    let ingredient = {
                        "name": this.name,
                        "default_unit_id": this.default_unit
                    }
                    this.foodStore.sendNewIngredient(ingredient)
                        .then((new_ingr) => {
                            this.$emit('created', new_ingr)
                            this.close()
                        })
                    .catch(err => {
                        handle_form_api_errors(err.response, this.errors, this.notifStore)
                    });
                })
                .catch(err => {
                    handle_form_local_errors(err.inner, this.errors, this.notifStore)
                });
        },
        save_unit_search() {
            this.unit_search = this.$refs.multiselect.search
        },
        set_unit(unit) {
            this.default_unit = unit.id
        },
        open() {
            this.opened = true
            setTimeout(() => {
                this.$refs.ingrName.focus()
                this.errors = {}
            }, 50)
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
        },
        validate(field) {
            validator.validateAt(field, this)
            .then(() => {
                this.errors[field] = null;
            })
            .catch(err => {
                setTimeout(() => this.errors[field] = err.message, 200)
            });
        }
    }
}
</script>

<style src="@vueform/multiselect/themes/default.css"></style>
