<script setup>
import { onMounted, nextTick, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useHead } from '@unhead/vue'
import { object, string } from 'yup'

import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const authStore = useAuthStore()
const notifStore = useNotifStore()
const router = useRouter()

useHead({
    title: 'Réinitialisation du mot de passe',
    meta: {
        name: 'robots',
        content: 'noindex',
    },
})

const props = defineProps({
    token: String,
})

const fields = ref({
    password: null,
})
const errors = ref({
    password: null,
})
const validator = object().shape({
    password: string()
        .required('Le mot de passe est obligatoire')
        .min(8, 'Le mot de passe doit faire au moins 8 caractères'),
})

const password_input_el = ref(null)
onMounted(async () => {
    await nextTick()
    password_input_el.value.focus()
})

async function send_password_reset_request() {
    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            authStore
                .reset_password(props.token, fields.value.password)
                .then(() => {
                    notifStore.show_info('Votre mot de passe a été modifié')
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
</script>

<template>
    <div class="my-8">
        <form
            class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
            @submit.prevent="send_password_reset_request"
        >
            <h1 class="text-xl font-bold text-center">Réinitialisation du mot de passe</h1>
            <div class="form-control">
                <label class="label">
                    <span class="label-text">Nouveau mot de passe</span>
                </label>
                <input
                    ref="password_input_el"
                    v-model="fields.password"
                    type="password"
                    class="input input-bordered w-full"
                    :class="errors.password && '!input-error'"
                />
                <label v-if="errors.password" class="label">
                    <span class="label-text-alt text-error">{{ errors.password }}</span>
                </label>
            </div>
            <div class="form-control">
                <button class="btn btn-primary w-full btn-lg">Valider</button>
            </div>
        </form>
    </div>
</template>
