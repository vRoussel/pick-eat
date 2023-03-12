<template>
    <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="updateAccount">
        <h1 class="text-xl font-bold text-center">Modification du compte</h1>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Mot de passe actuel</span>
            </label>
            <input
                v-model="old_password"
                type="password"
                placeholder="Requis"
                class="input input-bordered w-full"
                :class="errors.old_password && '!input-error'"
                @blur="validate('old_password')"
            >
            <label class="label" v-if="this.errors.old_password">
                <span class="label-text-alt text-error">{{ errors.old_password }}</span>
            </label>
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Nom</span>
            </label>
            <input
                v-model="name"
                type="text"
                class="input input-bordered w-full"
                :class="errors.name && '!input-error'"
                @blur="validate('name')"
            >
            <label class="label" v-if="this.errors.name">
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
            <label class="label" v-if="this.errors.email">
                <span class="label-text-alt text-error">{{ errors.email }}</span>
            </label>
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Nouveau mot de passe (si changement)</span>
            </label>
            <input
                v-model="password"
                type="password"
                placeholder="Laisser vide pour conserver l'actuel"
                class="input input-bordered w-full"
                :class="errors.password && '!input-error'"
                @blur="validate('password')"
            >
            <label class="label" v-if="this.errors.password">
                <span class="label-text-alt text-error">{{ errors.password }}</span>
            </label>
        </div>
        <button class="btn btn-primary w-full">
          Valider
        </button>
        <button
          type="button"
          class="btn btn-accent w-full"
          @click="cancel"
        >
          Annuler
        </button>
    </form>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'
import {handle_form_api_errors, handle_form_local_errors} from '@/utils/utils.js'
import { object, string } from "yup";

const validator = object().shape({
    email: string()
            .required("L'adresse email est obligatoire")
            .email("L'adresse email est invalide"),
    old_password: string()
            .required("Entrez votre mot de passe actuel"),
    password: string()
            .notRequired()
            .min(8, "Le mot de passe doit faire au moins 8 caractères")
            .nullable()
            .transform((value) => value ? value : null),
    name: string()
            .required("Le nom est obligatoire")
})

export default {
    name: 'AccountUpdateForm',
    data: function() {
        return {
            email: null,
            password: null,
            old_password: null,
            name: null,
            errors: {
                email: null,
                password: null,
                old_password: null,
                name: null
            }
        }
    },
    emits: ['done'],
    computed: {
        ...mapStores(useAuthStore, useNotifStore),
        account() {
            return this.authStore.account
        }
    },
    mounted() {
        this.email = this.account.email
        this.name = this.account.display_name
    },
    methods: {
        updateAccount() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {};
                    this.authStore.update_account(this.old_password, this.email, this.password, this.name).then(() => {
                        this.email = null
                        this.password = null
                        this.name = null
                        this.old_password = null
                        this.$emit('done')
                    })
                    .catch(err => {
                        handle_form_api_errors(err.response, this.errors, this.notifStore)
                    });
                })
                .catch(err => {
                    handle_form_local_errors(err.inner, this.errors, this.notifStore)
                });
        },
        cancel() {
            this.$emit('done')
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
