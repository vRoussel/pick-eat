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
        @submit.prevent="sendUnit"
      >
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Nom complet</span>
          </label>
          <input
            id="full_name"
            ref="unitName"
            v-model="full_name"
            class="input input-bordered w-full"
            type="text"
            name="full_name"
            :class="errors.full_name && '!input-error'"
            @blur="validate('full_name')"
          >
          <label class="label" v-if="this.errors.full_name">
              <span class="label-text-alt text-error">{{ errors.full_name }}</span>
          </label>
        </div>
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Abréviation</span>
          </label>
          <input
            id="short_name"
            v-model="short_name"
            class="input input-bordered w-full"
            type="text"
            name="short_name"
            :class="errors.short_name && '!input-error'"
          >
          <label class="label" v-if="this.errors.short_name">
              <span class="label-text-alt text-error">{{ errors.short_name }}</span>
          </label>
        </div>
        <div class="modal-action">
          <button
            class="btn btn-primary btn-sm btn-wide mx-auto"
          >Ajouter</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useFoodStore } from '@/store/food.js'
import { useNotifStore } from '@/store/notif.js'
import {handle_form_api_errors, handle_form_local_errors} from '@/utils/utils.js'
import { object, string } from "yup";

const validator = object().shape({
    full_name: string()
            .required("Le nom complet de l'unité est obligatoire"),
})

export default {
    name: 'NewUnitModal',
    props: {
        input: null
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            full_name: this.input,
            short_name: null,
            opened: false,
            errors: {
                full_name: null,
                short_name: null
            }
        }
    },
    computed: {
        ...mapStores(useFoodStore, useNotifStore),
    },
    watch: {
        input: function() {
            this.full_name = this.input;
        }
    },
    methods: {
        sendUnit() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    let unit = {
                        "full_name": this.full_name,
                        "short_name": this.short_name ? this.short_name : this.full_name,
                    }
                    this.foodStore.sendNewUnit(unit) .then((new_unit) => {
                            this.$emit('created', new_unit)
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
        open() {
            this.opened = true
            setTimeout(() => {
                this.$refs.unitName.focus()
                this.errors = {}
            }, 50)
        },
        close() {
            this.opened = false
            this.full_name = null
            this.short_name = null
            this.$emit('closed')
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
