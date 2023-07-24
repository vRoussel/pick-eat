<template>
  <div class="my-8">
    <form
      class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
      @submit.prevent="register"
    >
      <h1 class="text-xl font-bold text-center">
        Inscription
      </h1>
      <p class="text-center">
        Vous avez déjà un compte ? <router-link to="/login">
          Se connecter
        </router-link>
      </p>
      <div class="form-control">
        <label class="label">
          <span class="label-text">Pseudo</span>
        </label>
        <input
          ref="name"
          v-model="name"
          type="text"
          class="input input-bordered w-full"
          :class="errors.name && '!input-error'"
          @blur="validate('name')"
        >
        <label
          v-if="errors.name"
          class="label"
        >
          <span class="label-text-alt text-error">{{ errors.name }}</span>
        </label>
      </div>
      <div class="form-control">
        <label class="label">
          <span class="label-text">Adresse mail</span>
        </label>
        <input
          v-model="email"
          type="text"
          class="input input-bordered w-full"
          :class="errors.email && '!input-error'"
          @blur="validate('email')"
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
        </label>
        <input
          v-model="password"
          type="password"
          class="input input-bordered w-full"
          :class="errors.password && '!input-error'"
          @blur="validate('password')"
        >
        <label
          v-if="errors.password"
          class="label"
        >
          <span class="label-text-alt text-error">{{ errors.password }}</span>
        </label>
      </div>
      <div class="form-control">
        <button
          class="btn btn-primary w-full btn-lg"
          :class="{ loading: waiting_for_api }"
        >
          Créer mon compte
        </button>
      </div>
    </form>
  </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'
import Swal from 'sweetalert2'
import {handle_form_api_errors, handle_form_local_errors} from '@/utils/utils.js'
import { object, string } from "yup";

const validator = object().shape({
    email: string()
            .required("L'adresse email est obligatoire")
            .email("L'adresse email est invalide"),
    password: string()
            .required("Le mot de passe est obligatoire")
            .min(8, "Le mot de passe doit faire au moins 8 caractères"),
    name: string()
            .required("Le pseudo est obligatoire")
})

export default {
    name: 'Register',
    data: function() {
        return {
            email: null,
            password: null,
            name: null,
            waiting_for_api: false,
            errors: {
                email: null,
                password: null,
                name: null
            }
        }
    },
    computed: {
      ...mapStores(useAuthStore, useNotifStore)
    },
    mounted() {
        this.$nextTick(() => this.$refs.name.focus())
    },
    methods: {
        async register() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    this.waiting_for_api = true
                    this.authStore.register(this.email, this.password, this.name).then(() => {
                        Swal.fire({
                          title: 'Inscription (presque) terminée',
                          icon: 'success',
                          text: 'Cliquez sur le lien reçu par email pour valider votre inscription'
                        })
                        this.authStore.ask_account_validation_token(this.email)
                        this.$router.push('/login')
                        this.email = null
                        this.password = null
                        this.name = null
                    })
                    .catch(err => {
                        handle_form_api_errors(err.response, this.errors, this.notifStore)
                    });
                })
                .catch(err => {
                    handle_form_local_errors(err.inner, this.errors, this.notifStore)
                });
            this.waiting_for_api = false
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
