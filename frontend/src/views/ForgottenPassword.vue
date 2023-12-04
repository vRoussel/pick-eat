<template>
    <div class="my-8">
        <form
            class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
            @submit.prevent="send_password_reset_token_request"
        >
            <h1 class="text-xl font-bold text-center">Mot de passe oublié</h1>
            <p class="text-center">
                Entrez votre email pour recevoir un lien qui vous permettra de choisir un nouveau
                mot de passe.
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
                />
                <label v-if="errors.email" class="label">
                    <span class="label-text-alt text-error">{{ errors.email }}</span>
                </label>
            </div>
            <div class="form-control">
                <button class="btn btn-primary w-full btn-lg">Recevoir le lien</button>
            </div>
            <p class="text-center">
                <router-link to="/login"> Retour </router-link>
            </p>
        </form>
    </div>
</template>

<script>
import { mapStores } from 'pinia'
import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'
import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'
import { object, string } from 'yup'

const validator = object().shape({
    email: string().required("L'adresse mail est obligatoire").email("L'adresse mail est invalide"),
})

export default {
    name: 'ForgottenPassword',
    data: function () {
        return {
            email: null,
            errors: {
                email: null,
            },
        }
    },
    computed: {
        ...mapStores(useAuthStore, useNotifStore),
    },
    mounted() {
        this.$nextTick(() => this.$refs.email.focus())
    },
    methods: {
        async send_password_reset_token_request() {
            validator
                .validate(this, { abortEarly: false })
                .then(() => {
                    this.errors = {}
                    this.authStore
                        .ask_password_reset_token(this.email)
                        .then(() => {
                            this.notifStore.show_info(
                                "Si l'adresse mail correspond bien a votre compte, un email vous a été envoyé",
                            )
                            this.email = null
                            this.$router.push('/login')
                        })
                        .catch((err) => {
                            handle_form_api_errors(err.response, this.errors, this.notifStore)
                        })
                })
                .catch((err) => {
                    handle_form_local_errors(err.inner, this.errors, this.notifStore)
                })
        },
        validate(field) {
            validator
                .validateAt(field, this)
                .then(() => {
                    this.errors[field] = null
                })
                .catch((err) => {
                    setTimeout(() => (this.errors[field] = err.message), 200)
                })
        },
        resend_validation_email() {
            this.authStore.ask_account_validation_token(this.email)
            this.notifStore.show_info('Nouvel email de confirmation envoyé')
        },
    },
}
</script>
