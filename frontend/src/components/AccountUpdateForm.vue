<script setup>
import { onMounted, ref, nextTick } from 'vue'
import { object, string } from 'yup'

import { useAuthStore } from '@/store/auth.js'
import { useNotifStore } from '@/store/notif.js'

import { handle_form_api_errors, handle_form_local_errors } from '@/utils/utils.js'

const authStore = useAuthStore()
const notifStore = useNotifStore()

const emit = defineEmits(['done'])

const fields = ref({
    email: null,
    password: null,
    old_password: null,
    name: null,
})

const errors = ref({
    email: null,
    password: null,
    old_password: null,
    name: null,
})

const validator = object().shape({
    email: string()
        .required("L'adresse email est obligatoire")
        .email("L'adresse email est invalide"),
    old_password: string().required('Entrez votre mot de passe actuel'),
    password: string()
        .notRequired()
        .min(8, 'Le mot de passe doit faire au moins 8 caractères')
        .nullable()
        .transform((value) => (value ? value : null)),
    name: string().required('Le pseudo est obligatoire'),
})

const old_password_input_el = ref(null)
onMounted(async () => {
    fields.value.email = authStore.account.email
    fields.value.name = authStore.account.display_name
    await nextTick()
    old_password_input_el.value.focus()
})

function updateAccount() {
    errors.value = {}
    validator
        .validate(fields.value, { abortEarly: false })
        .then(() => {
            authStore
                .update_account(
                    fields.value.old_password,
                    fields.value.email,
                    fields.value.password,
                    fields.value.name,
                )
                .then(() => {
                    emit('done')
                })
                .catch((err) => {
                    handle_form_api_errors(err.response, errors.value, notifStore)
                })
        })
        .catch((err) => {
            handle_form_local_errors(err.inner, errors.value, notifStore)
        })
}

function cancel() {
    emit('done')
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
    <form
        class="mx-auto space-y-4 p-8 border-primary border-[1px] rounded-xl max-w-md"
        @submit.prevent="updateAccount"
    >
        <h1 class="text-xl font-bold text-center">Modification du compte</h1>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Mot de passe actuel</span>
            </label>
            <input
                ref="old_password_input_el"
                v-model="fields.old_password"
                type="password"
                placeholder="Requis"
                class="input input-bordered w-full"
                :class="errors.old_password && '!input-error'"
                @blur="validate('old_password')"
            />
            <label v-if="errors.old_password" class="label">
                <span class="label-text-alt text-error">{{ errors.old_password }}</span>
            </label>
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Nom</span>
            </label>
            <input
                v-model="fields.name"
                type="text"
                class="input input-bordered w-full"
                :class="errors.name && '!input-error'"
                @blur="validate('name')"
            />
            <label v-if="errors.name" class="label">
                <span class="label-text-alt text-error">{{ errors.name }}</span>
            </label>
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Adresse mail</span>
            </label>
            <input
                v-model="fields.email"
                type="text"
                class="input input-bordered w-full"
                :class="errors.email && '!input-error'"
                @blur="validate('email')"
            />
            <label v-if="errors.email" class="label">
                <span class="label-text-alt text-error">{{ errors.email }}</span>
            </label>
        </div>
        <div class="form-control">
            <label class="label">
                <span class="label-text">Nouveau mot de passe (si changement)</span>
            </label>
            <input
                v-model="fields.password"
                type="password"
                placeholder="Laisser vide pour conserver l'actuel"
                class="input input-bordered w-full"
                :class="errors.password && '!input-error'"
                @blur="validate('password')"
            />
            <label v-if="errors.password" class="label">
                <span class="label-text-alt text-error">{{ errors.password }}</span>
            </label>
        </div>
        <button class="btn btn-primary w-full">Valider</button>
        <button type="button" class="btn btn-accent w-full" @click="cancel">Annuler</button>
    </form>
</template>
