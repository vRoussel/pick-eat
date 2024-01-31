<script setup>
import { onMounted, nextTick, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useHead } from '@unhead/vue'
import { object, string } from 'yup'

import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const notifStore = useNotifStore()
const authStore = useAuthStore()
const router = useRouter()

useHead({
    title: 'Connection',
    meta: {
        name: 'robots',
        content: 'noindex'
    }
})

const fields = ref({
    email: null,
    password: null,
})
const errors = ref({
    email: null,
    password: null,
})
const validator = object().shape({
    email: string()
        .required("L'adresse mail est obligatoire")
        .email("L'adresse mail est invalide"),
    password: string()
        .required('Le mot de passe est obligatoire'),
})

const email_input_el = ref(null)
onMounted(async () => {
    await nextTick()
    email_input_el.value.focus()
})

const show_resend_validation_email = ref(false)
async function login() {
    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            authStore
                .login(fields.value.email, fields.value.password)
                .then(() => {
                    router.push(authStore.return_url || '/account')
                    authStore.return_url = null
                })
                .catch((err) => {
                    handle_form_api_errors(err.response, errors.value, notifStore)
                    if (
                        err.response.data.errors.find(
                            (x) => x.code == 'account_pending_validation',
                        )
                    )
                        show_resend_validation_email.value = true
                })
        })
        .catch((err) => {
            handle_form_local_errors(err.inner, errors.value, notifStore)
        })
}

function resend_validation_email() {
    show_resend_validation_email.value = false
    authStore.ask_account_validation_token(fields.value.email)
    notifStore.show_info('Nouvel email de confirmation envoyé')
}
</script>

<template>
    <div class="my-8">
        <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="login">
            <h1 class="text-xl font-bold text-center">Connexion</h1>
            <p class="text-center">
                Vous n'avez pas encore de compte ?
                <router-link to="/register" class="text-primary"> Créer un compte </router-link>
            </p>
            <p v-if="show_resend_validation_email" class="text-center">
                Vous n'avez pas reçu l'email de validation ?
                <span class="link link-primary no-underline" @click="resend_validation_email">Renvoyer</span>
            </p>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Adresse mail</span>
                </label>
                <input ref="email_input_el" v-model="fields.email" type="text" class="input input-bordered w-full"
                    :class="errors.email && '!input-error'" />
                <label v-if="errors.email" class="label">
                    <span class="label-text-alt text-error">{{ errors.email }}</span>
                </label>
            </div>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Mot de passe</span>
                    <router-link tabindex="-1" class="label-text-alt text-sm text-primary cursor-pointer"
                        to="/forgotten_password">Mot de passe oublié</router-link>
                </label>
                <input v-model="fields.password" type="password" class="input input-bordered w-full"
                    :class="errors.password && '!input-error'" />
                <label v-if="errors.password" class="label">
                    <span class="label-text-alt text-error">{{ errors.password }}</span>
                </label>
            </div>
            <div class="form-control">
                <button class="btn btn-primary w-full btn-lg">Connexion</button>
            </div>
        </form>
    </div>
</template>

