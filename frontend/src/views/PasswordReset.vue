<template>
  <div class="my-8">
    <form
      class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
      @submit.prevent="send_password_reset_request"
    >
      <h1 class="text-xl font-bold text-center">
        Réinitialisation du mot de passe
      </h1>
      <div class="form-control">
        <label class="label">
          <span class="label-text">Nouveau mot de passe</span>
        </label>
        <input
          ref="password"
          v-model="password"
          type="password"
          class="input input-bordered w-full"
          :class="errors.password && '!input-error'"
        >
        <label
          v-if="errors.password"
          class="label"
        >
          <span class="label-text-alt text-error">{{ errors.password }}</span>
        </label>
      </div>
      <div class="form-control">
        <button class="btn btn-primary w-full btn-lg">
          Valider
        </button>
      </div>
    </form>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'
import {handle_form_api_errors, handle_form_local_errors} from '@/utils/utils.js'
import { object, string } from "yup";

const validator = object().shape({
    password: string()
            .required("Le mot de passe est obligatoire")
            .min(8, "Le mot de passe doit faire au moins 8 caractères")
})

export default {
    name: 'PasswordReset',
    props: {
        token: {
            type: String
        }
    },
    data: function() {
        return {
            email: null,
            errors: {
                password: null
            }
        }
    },
    computed: {
      ...mapStores(useAuthStore, useNotifStore)
    },
    mounted() {
        this.$nextTick(() => this.$refs.password.focus())
    },
    methods: {
        async send_password_reset_request() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    this.authStore.reset_password(this.token, this.password).then(() => {
                        this.notifStore.show_info("Votre mot de passe a été modifié")
                        this.password = null
                        this.$router.push('/login')
                    })
                    .catch(err => {
                        handle_form_api_errors(err.response, this.errors, this.notifStore)
                    });
                })
                .catch(err => {
                    handle_form_local_errors(err.inner, this.errors, this.notifStore)
                });
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
