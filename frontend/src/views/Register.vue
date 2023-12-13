<script setup>
import { onMounted, nextTick, ref } from 'vue'
import { useRouter } from 'vue-router'
import Swal from 'sweetalert2'
import { object, string } from 'yup'

import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const authStore = useAuthStore()
const notifStore = useNotifStore()
const router = useRouter()

const validator = object().shape({
    email: string()
        .required("L'adresse email est obligatoire")
        .email("L'adresse email est invalide"),
    password: string()
        .required('Le mot de passe est obligatoire')
        .min(8, 'Le mot de passe doit faire au moins 8 caractères'),
    display_name: string().required('Le pseudo est obligatoire'),
})

const fields = ref({
    email: null,
    password: null,
    display_name: null
})
const errors = ref({
    email: null,
    password: null,
    display_name: null
})

const display_name_input_el = ref(null)
onMounted(async () => {
    await nextTick()
    display_name_input_el.value.focus()
})

async function register() {
    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            authStore
                .register(fields.value.email, fields.value.password, fields.value.display_name)
                .then(() => {
                    Swal.fire({
                        title: 'Inscription (presque) terminée',
                        icon: 'success',
                        text: 'Cliquez sur le lien reçu par email pour valider votre inscription',
                    })
                    authStore.ask_account_validation_token(fields.value.email)
                    router.push('/login')
                })
                .catch((err) => {
                    console.log(errors.value)
                    handle_form_api_errors(err.response, errors.value, notifStore)
                    console.log(errors.value)
                })
        })
        .catch((err) => {
            console.log(err)
            handle_form_local_errors(err.inner, errors.value, notifStore)
        })
}

function validate(field) {
    validator
        .validateAt(field, fields.value)
        .then(() => {
            errors.value[field] = null
        })
        .catch((err) => {
            setTimeout(() => (errors.value[field] = err.message), 200)
        })
}
</script>

<template>
    <div class="my-8">
        <form class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md" @submit.prevent="register">
            <h1 class="text-xl font-bold text-center">Inscription</h1>
            <p class="text-center">
                Vous avez déjà un compte ?
                <router-link class="text-primary" to="/login"> Se connecter </router-link>
            </p>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Pseudo</span>
                </label>
                <input ref="display_name_input_el" v-model="fields.display_name" type="text"
                    class="input input-bordered w-full" :class="errors.display_name && '!input-error'"
                    @blur="validate('display_name')" />
                <label v-if="errors.display_name" class="label">
                    <span class="label-text-alt text-error">{{ errors.display_name }}</span>
                </label>
            </div>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Adresse mail</span>
                </label>
                <input v-model="fields.email" type="text" class="input input-bordered w-full"
                    :class="errors.email && '!input-error'" @blur="validate('email')" />
                <label v-if="errors.email" class="label">
                    <span class="label-text-alt text-error">{{ errors.email }}</span>
                </label>
            </div>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Mot de passe</span>
                </label>
                <input v-model="fields.password" type="password" class="input input-bordered w-full"
                    :class="errors.password && '!input-error'" @blur="validate('password')" />
                <label v-if="errors.password" class="label">
                    <span class="label-text-alt text-error">{{ errors.password }}</span>
                </label>
            </div>
            <div class="form-control">
                <button class="btn btn-primary w-full btn-lg">
                    Créer mon compte
                </button>
            </div>
        </form>
    </div>
</template>
