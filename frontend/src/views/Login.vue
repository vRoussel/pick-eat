<template>
  <div class="my-8">
    <form
      class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
      @submit.prevent="login"
    >
      <h1 class="text-xl font-bold text-center">
        Connexion
      </h1>
      <p class="text-center">
        Vous n'avez pas encore de compte ? <router-link to="/register" class="text-primary">
          Créer un compte
        </router-link>
      </p>
      <p
        v-if="show_resend_validation_email"
        class="text-center"
      >
        Vous n'avez pas reçu l'email de validation ? <span
          class="link link-primary no-underline"
          @click="resend_validation_email"
        >Renvoyer</span>
      </p>
      <div class="form-control">
        <label class="label">
          <span class="label-text">Adresse mail</span>
        </label>
        <input
          ref="email"
          v-model="email"
          type="text"
          class="input input-bordered w-full"
          :class="errors.email && '!input-error'"
        >
        <label
          v-if="errors.email"
          class="label"
        >
          <span class="label-text-alt text-error">{{ errors.email }}</span>
        </label>
      </div>
      <div class="form-control">
        <label class="label">
          <span class="label-text">Mot de passe</span>
          <router-link
            tabindex="-1"
            class="label-text-alt text-sm text-primary cursor-pointer"
            to="/forgotten_password"
          >Mot de passe oublié</router-link>
        </label>
        <input
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
          Connexion
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
    email: string()
            .required("L'adresse mail est obligatoire")
            .email("L'adresse mail est invalide"),
    password: string()
            .required("Le mot de passe est obligatoire")
})

export default {
    name: 'Login',
    data: function() {
        return {
            email: null,
            password: null,
            show_resend_validation_email: false,
            errors: {
                email: null,
                password: null
            }
        }
    },
    computed: {
      ...mapStores(useAuthStore, useNotifStore)
    },
    mounted() {
        this.$nextTick(() => this.$refs.email.focus())
    },
    methods: {
        async login() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    this.authStore.login(this.email, this.password).then(() => {
                        this.$router.push(this.authStore.return_url || '/account')
                        this.authStore.return_url = null
                        this.email = null
                        this.password = null
                    })
                    .catch(err => {
                        handle_form_api_errors(err.response, this.errors, this.notifStore)
                        if (err.response.data.errors.find(x => x.key == "account_pending_validation"))
                            this.show_resend_validation_email = true
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
        },
        resend_validation_email() {
            this.show_resend_validation_email = false
            this.authStore.ask_account_validation_token(this.email)
            this.notifStore.show_info("Nouvel email de confirmation envoyé")
        }
    }
}
</script>
