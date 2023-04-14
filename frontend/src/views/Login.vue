<template>
    <div class="my-8">
        <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="login">
        <h1 class="text-xl font-bold text-center">Connexion</h1>
        <p class="text-center">Vous n'avez pas encore de compte ? <router-link to="/register">Créer un compte</router-link></p>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Adresse mail</span>
                </label>
                <input
                    v-model="email"
                    ref="email"
                    type="text"
                    class="input input-bordered w-full"
                    :class="errors.email && '!input-error'"
                >
                <label class="label" v-if="this.errors.email">
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
                >
                <label class="label" v-if="this.errors.password">
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
