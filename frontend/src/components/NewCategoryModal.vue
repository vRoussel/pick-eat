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
        @submit.prevent="sendCategory"
      >
        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">Nom</span>
          </label>
          <input
            id="name"
            ref="categName"
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
    name: string()
            .required("Le nom de la catégorie est obligatoire"),
})

export default {
    name: 'NewCategoryModal',
    computed: {
        ...mapStores(useFoodStore, useNotifStore),
    },
    emits: ['closed', 'created'],
    data: function() {
        return {
            name: null,
            opened: false,
            errors: {
                name: null
            }
        }
    },
    methods: {
        sendCategory() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    let category = {
                        "name": this.name,
                    }
                    this.foodStore.sendNewCategory(category) .then((new_categ) => {
                            this.$emit('created', new_categ)
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
            setTimeout(() =>  {
                this.$refs.categName.focus()
                this.errors = {}
            }, 50)
        },
        close() {
            this.opened = false
            this.name = ""
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
