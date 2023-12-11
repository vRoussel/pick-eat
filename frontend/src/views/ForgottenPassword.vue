<script setup>
import { onMounted, nextTick, ref } from 'vue'
import { useRouter } from 'vue-router'
import { object, string } from 'yup'

import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'


const notifStore = useNotifStore()
const authStore = useAuthStore()
const router = useRouter()

const fields = ref({
    email: null
})
const errors = ref({
    email: null
})
const validator = object().shape({
    email: string()
        .required("L'adresse mail est obligatoire")
        .email("L'adresse mail est invalide")
})

const email_input_el = ref(null)
onMounted(async () => {
    await nextTick()
    email_input_el.value.focus()
})


async function send_password_reset_token_request() {
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            errors.value = {}
            authStore
                .ask_password_reset_token(fields.value.email)
                .then(() => {
                    notifStore.show_info(
                        "Si l'adresse mail correspond bien a votre compte, un email vous a été envoyé",
                    )
                    fields.value.email = null
                    router.push('/login')
                })
                .catch((err) => {
                    handle_form_api_errors(err.response, errors.value, notifStore)
                })
        })
        .catch((err) => {
            handle_form_local_errors(err.inner, errors.value, notifStore)
        })
}

function resend_validation_email() {
    this.authStore.ask_account_validation_token(fields.value.email)
    this.notifStore.show_info('Nouvel email de confirmation envoyé')
}
</script>

<template>
    <div class="my-8">
        <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
            @submit.prevent="send_password_reset_token_request">
            <h1 class="text-xl font-bold text-center">Mot de passe oublié</h1>
            <p class="text-center">
                Entrez votre email pour recevoir un lien qui vous permettra de choisir un nouveau
                mot de passe.
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
                <button class="btn btn-primary w-full btn-lg">Recevoir le lien</button>
            </div>
            <p class="text-center">
                <router-link to="/login"> Retour </router-link>
            </p>
        </form>
    </div>
</template>
